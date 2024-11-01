// backwellApi/src/schedule_utils.rs

use crate::CourseSchedule;
use chrono::NaiveTime;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};

pub fn create_compatible_schedules(
    all_courses: &Vec<CourseSchedule>,
    course_names: &Vec<String>,
    _minimum: usize,
) -> Vec<Vec<CourseSchedule>> {
    let filtered_courses: Vec<&CourseSchedule> = all_courses.iter()
    .filter(|s| course_names.contains(&s.materia.nombre.trim().to_string()))
    .collect();

    let mut grouped_courses: HashMap<(String, i32), &CourseSchedule> = HashMap::new();
    for course in filtered_courses {
        let key = (course.materia.nombre.clone(), course.profesor.id);
        grouped_courses.insert(key.clone(), course);
    }

    let mut graph = Graph::<(String, i32), (), Undirected>::default();
    let mut node_indices = HashMap::new();

    // Corregido: eliminar '&' en el bucle
    for key in grouped_courses.keys() {
        let index = graph.add_node(key.clone());
        node_indices.insert(key.clone(), index);
    }

    let keys: Vec<(String, i32)> = grouped_courses.keys().cloned().collect();
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            let key_i = keys[i].clone();
            let key_j = keys[j].clone();

            if !courses_overlap(grouped_courses[&key_i], grouped_courses[&key_j]) {
                let index_i = node_indices[&key_i];
                let index_j = node_indices[&key_j];
                graph.add_edge(index_i, index_j, ());
            }
        }
    }

    let mut cliques = Vec::new();
    let mut r = Vec::new();
    let mut p: HashSet<_> = graph.node_indices().collect();
    let mut x = HashSet::new();
    bron_kerbosch(&graph, &mut r, &mut p, &mut x, &mut cliques);

    let mut final_schedules = Vec::new();
    for clique in cliques {
        let mut schedule_group = Vec::new();
        let mut materias_incluidas = HashSet::new();

        for node_index in &clique {
            let key = graph[*node_index].clone();
            let course = grouped_courses[&key].clone();
            schedule_group.push(course.clone());
            materias_incluidas.insert(course.materia.nombre.clone());
        }

        if materias_incluidas.len() == course_names.len() {
            final_schedules.push(schedule_group);
        }
    }

    final_schedules
}

fn bron_kerbosch(
    graph: &Graph<(String, i32), (), Undirected>,
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

fn courses_overlap(course1: &CourseSchedule, course2: &CourseSchedule) -> bool {
    for schedule1 in &course1.schedules {
        for schedule2 in &course2.schedules {
            if schedule1.dia == schedule2.dia {
                let start_time1 = parse_time(&schedule1.hora_inicio);
                let end_time1 = parse_time(&schedule1.hora_fin);
                let start_time2 = parse_time(&schedule2.hora_inicio);
                let end_time2 = parse_time(&schedule2.hora_fin);

                if let (Some(start_time1), Some(end_time1), Some(start_time2), Some(end_time2)) =
                    (start_time1, end_time1, start_time2, end_time2)
                {
                    if (start_time1 < end_time2) && (end_time1 > start_time2) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn parse_time(time_str: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time_str, "%H:%M:%S").ok()
        .or_else(|| NaiveTime::parse_from_str(time_str, "%H:%M").ok())
}
