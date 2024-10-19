// src/utils/schedule_generator_utils.rs

use diesel::prelude::*;
use crate::models::{Schedule, Course};
use crate::schema::{schedules, courses};
use std::collections::{HashMap, HashSet};
use chrono::NaiveTime;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleGroup {
    pub course_id: i32,
    pub professor_id: i32,
    pub schedules: Vec<Schedule>,
}

pub fn create_compatible_schedules(
    courses_list: Vec<String>,
    minimum: usize,
    conn: &mut PgConnection,
) -> Result<Vec<Vec<ScheduleGroup>>, diesel::result::Error> {
    let mut grouped_schedules: HashMap<i32, HashMap<i32, Vec<Schedule>>> = HashMap::new();

    // Fetch schedules for the selected courses
    for course_name in courses_list {
        let course = courses::table
            .filter(courses::name.eq(&course_name))
            .first::<Course>(conn)?;

        let schedule_list = schedules::table
            .filter(schedules::course_id.eq(course.id))
            .select(Schedule::as_select())
            .load::<Schedule>(conn)?;

        let mut professor_map: HashMap<i32, Vec<Schedule>> = HashMap::new();
        for schedule in schedule_list {
            professor_map
                .entry(schedule.professor_id)
                .or_insert_with(Vec::new)
                .push(schedule);
        }

        grouped_schedules.insert(course.id, professor_map);
    }

    // Create the graph
    let mut graph = Graph::<(i32, i32), (), Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Add nodes
    for (course_id, professor_map) in &grouped_schedules {
        for (professor_id, _schedules) in professor_map {
            let node_index = graph.add_node((*course_id, *professor_id));
            node_indices.insert((*course_id, *professor_id), node_index);
        }
    }

    // Add edges between compatible nodes
    let node_keys: Vec<(i32, i32)> = node_indices.keys().cloned().collect();
    for i in 0..node_keys.len() {
        for j in (i + 1)..node_keys.len() {
            let node_a = node_keys[i];
            let node_b = node_keys[j];

            // Ensure they are not from the same course
            if node_a.0 != node_b.0 {
                let schedules_a = &grouped_schedules[&node_a.0][&node_a.1];
                let schedules_b = &grouped_schedules[&node_b.0][&node_b.1];

                if !schedules_overlap(schedules_a, schedules_b) {
                    let index_a = node_indices[&node_a];
                    let index_b = node_indices[&node_b];
                    graph.add_edge(index_a, index_b, ());
                }
            }
        }
    }

    // Implement Bron-Kerbosch algorithm
    let cliques = find_cliques(&graph, minimum);

    // Build the compatible schedules from the cliques
    let mut compatible_schedules = Vec::new();
    for clique in cliques {
        let mut schedule_group = Vec::new();
        for node_index in clique {
            let (course_id, professor_id) = graph[node_index];
            let schedules = grouped_schedules[&course_id][&professor_id].clone();
            schedule_group.push(ScheduleGroup {
                course_id,
                professor_id,
                schedules,
            });
        }
        compatible_schedules.push(schedule_group);
    }

    Ok(compatible_schedules)
}

fn find_cliques(
    graph: &Graph<(i32, i32), (), Undirected>,
    minimum_size: usize,
) -> Vec<Vec<NodeIndex>> {
    let mut cliques = Vec::new();
    let all_nodes: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut potential_clique = Vec::new();
    let mut candidates = all_nodes.clone();
    let mut already_found = HashSet::new();

    bron_kerbosch(
        graph,
        &mut cliques,
        &mut potential_clique,
        &candidates,
        &already_found,
        minimum_size,
    );

    cliques
}

fn bron_kerbosch(
    graph: &Graph<(i32, i32), (), Undirected>,
    cliques: &mut Vec<Vec<NodeIndex>>,
    potential_clique: &mut Vec<NodeIndex>,
    candidates: &HashSet<NodeIndex>,
    already_found: &HashSet<NodeIndex>,
    minimum_size: usize,
) {
    if candidates.is_empty() && already_found.is_empty() {
        if potential_clique.len() >= minimum_size {
            cliques.push(potential_clique.clone());
        }
        return;
    }

    let mut candidates = candidates.clone();
    while let Some(node) = candidates.iter().next().cloned() {
        potential_clique.push(node);

        let neighbors: HashSet<NodeIndex> = graph.neighbors(node).collect();
        let new_candidates = candidates.intersection(&neighbors).cloned().collect();
        let new_already_found = already_found.intersection(&neighbors).cloned().collect();

        bron_kerbosch(
            graph,
            cliques,
            potential_clique,
            &new_candidates,
            &new_already_found,
            minimum_size,
        );

        potential_clique.pop();
        candidates.remove(&node);
        let mut already_found = already_found.clone();
        already_found.insert(node);
    }
}

fn schedules_overlap(schedules_a: &[Schedule], schedules_b: &[Schedule]) -> bool {
    for sa in schedules_a {
        for sb in schedules_b {
            if sa.day == sb.day {
                if times_overlap(sa.start_time, sa.end_time, sb.start_time, sb.end_time) {
                    return true;
                }
            }
        }
    }
    false
}

fn times_overlap(
    start_a: NaiveTime,
    end_a: NaiveTime,
    start_b: NaiveTime,
    end_b: NaiveTime,
) -> bool {
    (start_a < end_b) && (end_a > start_b)
}
