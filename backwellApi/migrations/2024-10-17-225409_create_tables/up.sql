-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- CREATE TABLE IF NOT EXISTS for users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(200) NOT NULL,
    is_staff BOOLEAN NOT NULL DEFAULT FALSE
);

-- CREATE TABLE IF NOT EXISTS for professors
CREATE TABLE IF NOT EXISTS professors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

-- CREATE TABLE IF NOT EXISTS for rooms
CREATE TABLE IF NOT EXISTS rooms (
    id SERIAL PRIMARY KEY,
    room_number VARCHAR(50) NOT NULL,
    capacity INTEGER NOT NULL
);

-- CREATE TABLE IF NOT EXISTS for courses
CREATE TABLE IF NOT EXISTS courses (
    id SERIAL PRIMARY KEY,
    course_id VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    catalog_number VARCHAR(50) NOT NULL
);

-- CREATE TABLE IF NOT EXISTS for schedules
CREATE TABLE IF NOT EXISTS schedules (
    id SERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    professor_id INTEGER NOT NULL REFERENCES professors(id),
    room_id INTEGER NOT NULL REFERENCES rooms(id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    day VARCHAR(1) NOT NULL,
    modality VARCHAR(10) NOT NULL
);
