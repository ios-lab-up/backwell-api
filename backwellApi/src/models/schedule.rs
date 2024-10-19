// src/models/schedule.rs

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveTime};
use crate::schema::schedules;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = schedules)]
pub struct Schedule {
    pub id: i32,
    pub course_id: i32,
    pub professor_id: i32,
    pub room_id: Option<i32>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub modality: String,
    pub day: String,
}
