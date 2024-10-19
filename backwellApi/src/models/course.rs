// src/models/course.rs

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::courses;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub course_id: String,
    pub name: String,
    pub catalog_number: String,
}
