// src/schedule_utils.rs

use crate::CourseSchedule;
use chrono::NaiveTime;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};

pub fn create_compatible_schedules(
    all_schedules: &Vec<CourseSchedule>,
    course_names: &Vec<String>,
    _minimum: usize,
) -> Vec<Vec<CourseSchedule>> {
    // Filter schedules for the requested courses
    let filtered_schedules: Vec<&CourseSchedule> = all_schedules.iter()
        .filter(|s| course_names.contains(&s.materia.nombre.trim().to_string()))
        .collect();

    // Group schedules by course and professor
    let mut grouped_schedules: HashMap<(i32, i32), Vec<&CourseSchedule>> = HashMap::new();
    for schedule in filtered_schedules {
        let key = (schedule.materia.id, schedule.profesor.id);
        grouped_schedules.entry(key).or_insert(Vec::new()).push(schedule);
    }

    // Build the graph
    let mut graph = Graph::<(i32, i32), (), Undirected>::default();
    let mut node_indices = HashMap::new();

    for &key in grouped_schedules.keys() {
        let index = graph.add_node(key);
        node_indices.insert(key, index);
    }

    // Add edges between non-overlapping schedules
    let keys: Vec<(i32, i32)> = grouped_schedules.keys().cloned().collect();
    for i in 0..keys.len() {
        for j in (i+1)..keys.len() {
            let key_i = keys[i];
            let key_j = keys[j];
            if !schedules_overlap(&grouped_schedules[&key_i], &grouped_schedules[&key_j]) {
                let index_i = node_indices[&key_i];
                let index_j = node_indices[&key_j];
                graph.add_edge(index_i, index_j, ());
            }
        }
    }

    // Find cliques using Bron-Kerbosch algorithm
    let mut cliques = Vec::new();
    let mut r = Vec::new();
    let mut p: HashSet<_> = graph.node_indices().collect();
    let mut x = HashSet::new();
    bron_kerbosch(&graph, &mut r, &mut p, &mut x, &mut cliques);

    // Collect schedules from cliques
    let mut final_schedules = Vec::new();
    for clique in cliques {
        let mut schedule_group = Vec::new();
        for node_index in clique {
            let key = graph[node_index];
            let schedules = grouped_schedules[&key].clone();
            schedule_group.extend(schedules.into_iter().cloned());
        }
        final_schedules.push(schedule_group);
    }

    final_schedules
}

fn bron_kerbosch(
    graph: &Graph<(i32, i32), (), Undirected>,
    r: &mut Vec<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
    cliques: &mut Vec<Vec<NodeIndex>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    } else {
        let mut p_vec: Vec<NodeIndex> = p.iter().cloned().collect();
        while let Some(v) = p_vec.pop() {
            r.push(v);
            let neighbors: HashSet<_> = graph.neighbors(v).collect();
            let mut p_new = p.intersection(&neighbors).cloned().collect();
            let mut x_new = x.intersection(&neighbors).cloned().collect();
            bron_kerbosch(graph, r, &mut p_new, &mut x_new, cliques);
            r.pop();
            p.remove(&v);
            x.insert(v);
        }
    }
}

fn schedules_overlap(schedules1: &Vec<&CourseSchedule>, schedules2: &Vec<&CourseSchedule>) -> bool {
    for s1 in schedules1 {
        for s2 in schedules2 {
            let days1 = get_days(s1);
            let days2 = get_days(s2);

            let common_days: HashSet<&str> = days1.intersection(&days2).cloned().collect();
            if !common_days.is_empty() {
                let start_time1 = NaiveTime::parse_from_str(&s1.hora_inicio, "%H:%M:%S").unwrap();
                let end_time1 = NaiveTime::parse_from_str(&s1.hora_fin, "%H:%M:%S").unwrap();
                let start_time2 = NaiveTime::parse_from_str(&s2.hora_inicio, "%H:%M:%S").unwrap();
                let end_time2 = NaiveTime::parse_from_str(&s2.hora_fin, "%H:%M:%S").unwrap();

                if (start_time1 < end_time2) && (end_time1 > start_time2) {
                    return true;
                }
            }
        }
    }
    false
}

fn get_days(schedule: &CourseSchedule) -> HashSet<&str> {
    let mut days = HashSet::new();
    if schedule.lunes { days.insert("L"); }
    if schedule.martes { days.insert("M"); }
    if schedule.miercoles { days.insert("W"); }
    if schedule.jueves { days.insert("J"); }
    if schedule.viernes { days.insert("V"); }
    if schedule.sabado { days.insert("S"); }
    if schedule.domingo { days.insert("D"); }
    days
}
