use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDateTime, NaiveDate};

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

impl User {
    // Aquí puedes agregar métodos para manejar el hashing de contraseñas
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = professors)]
pub struct Professor {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: i32,
    pub room_number: String,
    pub capacity: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub course_id: String,
    pub name: String,
    pub catalog_number: String,
}
use super::{Course, Professor, Room};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Associations)]
#[diesel(belongs_to(Course))]
#[diesel(belongs_to(Professor))]
#[diesel(belongs_to(Room))]
#[diesel(table_name = schedules)]
pub struct Schedule {
    pub id: i32,
    pub course_id: i32,
    pub professor_id: i32,
    pub room_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub day: String,
    pub modality: String,
}
impl User {
    pub fn hash_password(plain: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(plain, bcrypt::DEFAULT_COST)
    }

    pub fn verify_password(&self, plain: &str) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(plain, &self.password)
    }
}
