use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("¡Hola desde Rust y Actix-web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurado en .env o variables de entorno");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("No se pudo crear el pool de conexiones");

    if args.len() > 1 && args[1] == "migrate" {
        // Ejecutar migraciones pendientes
        println!("Ejecutando migraciones pendientes...");
        let mut conn = pool.get().expect("No se pudo obtener la conexión del pool");
        match conn.run_pending_migrations(MIGRATIONS) {
            Ok(migrations) => {
                println!("Migraciones ejecutadas: {:?}", migrations);
            }
            Err(e) => {
                eprintln!("Error al ejecutar migraciones: {}", e);
            }
        }
        return Ok(());
    }

    println!("Servidor corriendo en http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
