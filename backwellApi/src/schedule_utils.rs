// backwellApi/src/schedule_utils.rs

use crate::CourseSchedule;
use std::collections::{HashMap, HashSet};

const MAX_COMBINATIONS: usize = 100;

pub fn create_compatible_schedules(
    all_courses: &Vec<CourseSchedule>,
    course_names: &Vec<String>,
    professors: &Option<Vec<String>>,
    minimum: usize,
) -> Vec<Vec<CourseSchedule>> {
    let mut courses_by_name: HashMap<String, Vec<&CourseSchedule>> = HashMap::new();

    for course in all_courses {
        let name = course.materia.nombre.trim().to_string();
        if course_names.contains(&name) {
            courses_by_name.entry(name).or_default().push(course);
        }
    }

    // Collect professors present in data
    let mut professors_in_data = HashSet::new();
    for course in all_courses {
        if let Some(prof) = &course.profesor {
            if let Some(prof_name) = &prof.nombre {
                professors_in_data.insert(prof_name.trim().to_string());
            }
        }
    }

    if let Some(professors_list) = professors {
        let professors_list_trimmed: HashSet<String> =
            professors_list.iter().map(|p| p.trim().to_string()).collect();
        let missing_professors: HashSet<_> = professors_list_trimmed
            .difference(&professors_in_data)
            .cloned()
            .collect();
        if !missing_professors.is_empty() {
            return Vec::new();
        }
    }

    let mut course_lists: Vec<Vec<&CourseSchedule>> = Vec::new();
    for name in course_names {
        if let Some(courses) = courses_by_name.get(name) {
            course_lists.push(courses.clone());
        } else {
            return Vec::new();
        }
    }

    let mut results = Vec::new();
    let mut current_combination = Vec::new();

    generate_combinations(
        &course_lists,
        &mut current_combination,
        0,
        &mut results,
        professors,
        minimum,
    );

    results
}

fn generate_combinations<'a>(
    course_lists: &'a [Vec<&'a CourseSchedule>],
    current_combination: &mut Vec<&'a CourseSchedule>,
    index: usize,
    results: &mut Vec<Vec<CourseSchedule>>,
    professors: &Option<Vec<String>>,
    minimum: usize,
) {
    if results.len() >= MAX_COMBINATIONS {
        return;
    }

    if index == course_lists.len() {
        if current_combination.len() >= minimum {
            if let Some(professors_list) = professors {
                let mut professors_in_combination = HashSet::new();
                for course in current_combination.iter() {
                    if let Some(prof) = &course.profesor {
                        if let Some(prof_name) = &prof.nombre {
                            professors_in_combination.insert(prof_name.trim().to_string());
                        }
                    }
                }
                let professors_list_trimmed: HashSet<String> =
                    professors_list.iter().map(|p| p.trim().to_string()).collect();
                if !professors_list_trimmed.is_subset(&professors_in_combination) {
                    return;
                }
            }
            results.push(
                current_combination
                    .iter()
                    .map(|&course| course.clone())
                    .collect(),
            );
        }
        return;
    }

    for &course in &course_lists[index] {
        if !conflicts_with_current_combination(course, current_combination) {
            current_combination.push(course);
            generate_combinations(
                course_lists,
                current_combination,
                index + 1,
                results,
                professors,
                minimum,
            );
            current_combination.pop();
        }
    }

    if current_combination.len() + (course_lists.len() - index - 1) >= minimum {
        generate_combinations(
            course_lists,
            current_combination,
            index + 1,
            results,
            professors,
            minimum,
        );
    }
}

fn conflicts_with_current_combination(
    course: &CourseSchedule,
    current_combination: &Vec<&CourseSchedule>,
) -> bool {
    for &existing_course in current_combination {
        if courses_overlap(course, existing_course) {
            return true;
        }
    }
    false
}

fn courses_overlap(course1: &CourseSchedule, course2: &CourseSchedule) -> bool {
    for schedule1 in &course1.schedules {
        for schedule2 in &course2.schedules {
            if schedule1.dia == schedule2.dia {
                let start_time1 = parse_time(&schedule1.hora_inicio);
                let end_time1 = parse_time(&schedule1.hora_fin);
                let start_time2 = parse_time(&schedule2.hora_inicio);
                let end_time2 = parse_time(&schedule2.hora_fin);

                if let (Some(start1), Some(end1), Some(start2), Some(end2)) =
                    (start_time1, end_time1, start_time2, end_time2)
                {
                    if (start1 < end2) && (end1 > start2) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn parse_time(time_str: &str) -> Option<chrono::NaiveTime> {
    chrono::NaiveTime::parse_from_str(time_str, "%H:%M:%S")
        .ok()
        .or_else(|| chrono::NaiveTime::parse_from_str(time_str, "%H:%M").ok())
}
