// src/models/user.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;

// Definir manualmente la tabla users
table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        is_staff -> Bool,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub is_staff: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub is_staff: bool,
}
