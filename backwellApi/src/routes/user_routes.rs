// src/routes/user_routes.rs

use actix_web::web;
use crate::handlers::user_handlers;
use crate::utils::auth_middleware::auth_middleware;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/login", web::post().to(user_handlers::login))
            .service(
                web::scope("")
                    .wrap(auth_middleware())
                    .route("/", web::get().to(user_handlers::get_users))
                    .route("/create", web::post().to(user_handlers::create_user))
                    .route("/read/{id}", web::get().to(user_handlers::get_user))
                    .route("/update/{id}", web::put().to(user_handlers::update_user))
                    .route("/delete/{id}", web::delete().to(user_handlers::delete_user)),
            ),
    );
}
