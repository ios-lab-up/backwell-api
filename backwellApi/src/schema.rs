// src/schema.rs

// Import Diesel macros
use diesel::prelude::*;
use diesel::table;
use diesel::joinable;
use diesel::allow_tables_to_appear_in_same_query;

// Define the tables using the `table!` macro
table! {
    courses (id) {
        id -> Int4,
        course_id -> Varchar,
        name -> Varchar,
        catalog_number -> Varchar,
    }
}

table! {
    professors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        room_number -> Varchar,
        capacity -> Int4,
    }
}

table! {
    schedules (id) {
        id -> Int4,
        course_id -> Int4,
        professor_id -> Int4,
        room_id -> Nullable<Int4>,
        start_date -> Date,
        end_date -> Date,
        start_time -> Time,
        end_time -> Time,
        modality -> Varchar,
        day -> Varchar,
    }
}

// Define relationships between tables
joinable!(schedules -> courses (course_id));
joinable!(schedules -> professors (professor_id));
joinable!(schedules -> rooms (room_id));

// Allow tables to appear together in queries
allow_tables_to_appear_in_same_query!(
    courses,
    professors,
    rooms,
    schedules,
);
