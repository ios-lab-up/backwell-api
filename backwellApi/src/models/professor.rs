// src/models/professor.rs

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::professors;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = professors)]
pub struct Professor {
    pub id: i32,
    pub name: String,
}
