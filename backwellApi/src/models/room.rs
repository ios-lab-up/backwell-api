// src/models/room.rs

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::rooms;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: i32,
    pub room_number: String,
    pub capacity: i32,
}
