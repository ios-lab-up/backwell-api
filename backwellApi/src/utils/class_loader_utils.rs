// src/utils/class_loader_utils.rs
use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;
use diesel::prelude::*;
use crate::models::{Course, Professor, Room, Schedule};
use crate::schema::{courses, professors, rooms, schedules};
use chrono::{NaiveDate, NaiveTime};

pub fn process_excel_data(
    path: &Path,
    conn: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        // Process the range
        for row in range.rows().skip(1) {
            let professor_name = row.get(0)
                .and_then(|cell| cell.get_string())
                .unwrap_or("")
                .to_string();
            let course_name: String = row[1].get_string().unwrap_or("").to_string();
            let catalog_number: String = row[2].get_string().unwrap_or("").to_string();
            let modality: String = row[3].get_string().unwrap_or("").to_string();
            let day: String = row[4].get_string().unwrap_or("").to_string();
            let start_time_str: String = row[5].get_string().unwrap_or("").to_string();
            let end_time_str: String = row[6].get_string().unwrap_or("").to_string();
            let start_date_str: String = row[7].get_string().unwrap_or("").to_string();
            let end_date_str: String = row[8].get_string().unwrap_or("").to_string();
            let room_number: String = row[9].get_string().unwrap_or("").to_string();
            let capacity: i32 = row[10].get_float().unwrap_or(0.0) as i32;

            // Parse dates and times
            let start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")?;
            let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")?;
            let start_time = NaiveTime::parse_from_str(&start_time_str, "%H:%M:%S")?;
            let end_time = NaiveTime::parse_from_str(&end_time_str, "%H:%M:%S")?;

            // Handle Professor
            let professor = professors::table
                .filter(professors::name.eq(&professor_name))
                .first::<Professor>(conn)
                .optional()?;

            let professor = match professor {
                Some(p) => p,
                None => {
                    diesel::insert_into(professors::table)
                        .values(&Professor { id: 0, name: professor_name.clone() })
                        .get_result::<Professor>(conn)?
                }
            };

            // Handle Course
            let course = courses::table
                .filter(courses::name.eq(&course_name))
                .first::<Course>(conn)
                .optional()?;

            let course = match course {
                Some(c) => c,
                None => {
                    diesel::insert_into(courses::table)
                        .values(&Course {
                            id: 0,
                            course_id: uuid::Uuid::new_v4().to_string(),
                            name: course_name.clone(),
                            catalog_number: catalog_number.clone(),
                        })
                        .get_result::<Course>(conn)?
                }
            };

            // Handle Room
            let room = if !room_number.is_empty() {
                Some(
                    rooms::table
                        .filter(rooms::room_number.eq(&room_number))
                        .first::<Room>(conn)
                        .optional()?
                        .unwrap_or_else(|| {
                            diesel::insert_into(rooms::table)
                                .values(&Room {
                                    id: 0,
                                    room_number: room_number.clone(),
                                    capacity,
                                })
                                .get_result::<Room>(conn)
                                .unwrap()
                        }),
                )
            } else {
                eprintln!("Sheet1 not found or could not be read");
                return Err(Box::from("Sheet1 not found or could not be read"));
            
            };

            // Insert Schedule
            let new_schedule = Schedule {
                id: 0,
                course_id: course.id,
                professor_id: professor.id,
                room_id: room.map(|r| r.id),
                start_date,
                end_date,
                start_time,
                end_time,
                modality: modality.clone(),
                day: day.clone(),
            };

            diesel::insert_into(schedules::table)
                .values(&new_schedule)
                .execute(conn)?;
        }
    }

    Ok(())
}
