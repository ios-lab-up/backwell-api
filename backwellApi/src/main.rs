// src/main.rs

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

mod handlers;
mod models;
mod routes;
mod utils;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env
    dotenv().ok();

    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the database connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start the HTTP server
    println!("Starting server at http://0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Add the pool to the application data
            .configure(routes::init_routes) // Configure routes
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
