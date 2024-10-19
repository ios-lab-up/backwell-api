// src/routes/schedule_generator_routes.rs

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    use crate::handlers::schedule_generator_handlers;

    cfg.service(
        web::scope("/schedules")
            .route("/generate", web::post().to(schedule_generator_handlers::generate_schedule)),
    );
}
