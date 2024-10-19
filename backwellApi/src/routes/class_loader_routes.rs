// src/routes/class_loader_routes.rs

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    use crate::handlers::class_loader_handlers;

    cfg.service(
        web::scope("/classes")
            .route("/load", web::post().to(class_loader_handlers::load_course_list))
            .route("/get_schedules", web::get().to(class_loader_handlers::get_schedules)),
    );
}
