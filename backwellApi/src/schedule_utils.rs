// backwellApi/src/schedule_utils.rs

use crate::CourseSchedule;
use std::collections::HashMap;

/// Maximum number of schedule combinations to generate to prevent performance issues.
const MAX_COMBINATIONS: usize = 10;

/// Generates compatible schedules based on the available courses, requested course names, and minimum requirement.
/// Utilizes a recursive backtracking algorithm to efficiently build valid schedule combinations.
///
/// # Arguments
///
/// * `all_courses` - A reference to a vector of `CourseSchedule` representing all available courses.
/// * `course_names` - A reference to a vector of `String` representing the names of courses to include.
/// * `minimum` - The minimum number of courses required in a valid schedule.
///
/// # Returns
///
/// * `Vec<Vec<CourseSchedule>>` - A vector of compatible schedules, each being a vector of `CourseSchedule`.
pub fn create_compatible_schedules(
    all_courses: &Vec<CourseSchedule>,
    course_names: &Vec<String>,
    minimum: usize,
) -> Vec<Vec<CourseSchedule>> {
    // Group courses by their names for easier combination generation
    let mut courses_by_name: HashMap<String, Vec<&CourseSchedule>> = HashMap::new();
    for course in all_courses {
        let name = course.materia.nombre.trim().to_string();
        if course_names.contains(&name) {
            courses_by_name.entry(name).or_default().push(course);
        }
    }

    // Prepare a list of course groups corresponding to each requested course name
    let mut course_lists: Vec<Vec<&CourseSchedule>> = Vec::new();
    for name in course_names {
        if let Some(courses) = courses_by_name.get(name) {
            course_lists.push(courses.clone());
        }
    }

    // Initialize variables for backtracking
    let mut results = Vec::new();
    let mut current_combination = Vec::new();

    // Start generating combinations
    generate_combinations(
        &course_lists,
        &mut current_combination,
        0,
        &mut results,
        minimum,
    );

    results
}

/// Recursive function to generate valid schedule combinations using backtracking.
///
/// # Arguments
///
/// * `course_lists` - A reference to a vector of vectors containing references to `CourseSchedule` objects.
/// * `current_combination` - A mutable reference to the current combination being built.
/// * `index` - The current index in `course_lists` being processed.
/// * `results` - A mutable reference to the vector storing all valid schedule combinations.
/// * `minimum` - The minimum number of courses required in a valid schedule.
fn generate_combinations<'a>(
    course_lists: &'a Vec<Vec<&'a CourseSchedule>>,
    current_combination: &mut Vec<&'a CourseSchedule>,
    index: usize,
    results: &mut Vec<Vec<CourseSchedule>>,
    minimum: usize,
) {
    // Terminate early if the maximum number of combinations is reached
    if results.len() >= MAX_COMBINATIONS {
        return;
    }

    // Base case: All course groups have been processed
    if index == course_lists.len() {
        if current_combination.len() >= minimum {
            // Clone the current combination and add to results
            results.push(current_combination.iter().cloned().cloned().collect());
        }
        return;
    }

    // Iterate through each course in the current course group
    for &course in &course_lists[index] {
        if !conflicts_with_current_combination(course, current_combination) {
            // Add course to the current combination
            current_combination.push(course);
            // Recurse to process the next course group
            generate_combinations(
                course_lists,
                current_combination,
                index + 1,
                results,
                minimum,
            );
            // Remove the course to backtrack
            current_combination.pop();
        }
    }

    // Optionally skip the current course group to meet the minimum requirement
    generate_combinations(
        course_lists,
        current_combination,
        index + 1,
        results,
        minimum,
    );
}

/// Checks if adding a course to the current combination would cause a schedule conflict.
///
/// # Arguments
///
/// * `course` - A reference to the `CourseSchedule` being considered.
/// * `current_combination` - A reference to the current combination of courses.
///
/// # Returns
///
/// * `bool` - `true` if there is a conflict; otherwise, `false`.
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

/// Determines if two courses have overlapping schedules.
///
/// # Arguments
///
/// * `course1` - A reference to the first `CourseSchedule`.
/// * `course2` - A reference to the second `CourseSchedule`.
///
/// # Returns
///
/// * `bool` - `true` if there is any overlapping schedule; otherwise, `false`.
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
                    // Check if time intervals overlap
                    if (start1 < end2) && (end1 > start2) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Parses a time string into a `NaiveTime` object.
///
/// # Arguments
///
/// * `time_str` - A string slice representing the time (e.g., "08:30:00").
///
/// # Returns
///
/// * `Option<chrono::NaiveTime>` - `Some(NaiveTime)` if parsing is successful; otherwise, `None`.
fn parse_time(time_str: &str) -> Option<chrono::NaiveTime> {
    chrono::NaiveTime::parse_from_str(time_str, "%H:%M:%S").ok()
        .or_else(|| chrono::NaiveTime::parse_from_str(time_str, "%H:%M").ok())
}
