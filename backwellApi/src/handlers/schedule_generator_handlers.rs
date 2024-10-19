// src/handlers/schedule_generator_handlers.rs

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::utils::schedule_generator_utils::create_compatible_schedules;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize)]
pub struct GenerateScheduleRequest {
    pub courses: Vec<String>,
    pub minimum: usize,
}

pub async fn generate_schedule(
    pool: web::Data<DbPool>,
    request: web::Json<GenerateScheduleRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let result = web::block(move || {
        create_compatible_schedules(
            request.courses.clone(),
            request.minimum,
            &mut conn,
        )
    })
    .await;

    match result {
        Ok(Ok(schedules)) => HttpResponse::Ok().json(schedules),
        Ok(Err(e)) => {
            eprintln!("Error generating schedules: {:?}", e);
            HttpResponse::InternalServerError().body("Error generating schedules")
        }
        Err(e) => {
            eprintln!("Error executing web::block: {:?}", e);
            HttpResponse::InternalServerError().body("Error generating schedules")
        }
    }
}
