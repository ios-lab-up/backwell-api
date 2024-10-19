// src/routes/mod.rs

use actix_web::web;

mod class_loader_routes;
mod schedule_generator_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    class_loader_routes::init_routes(cfg);
    schedule_generator_routes::init_routes(cfg);
}
