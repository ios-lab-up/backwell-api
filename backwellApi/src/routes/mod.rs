// src/routes/mod.rs

use actix_web::web;

mod class_loader_routes;
mod schedule_generator_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    schedule_generator_routes::init_routes(cfg);
}
