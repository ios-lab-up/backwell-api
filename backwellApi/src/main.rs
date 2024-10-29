// src/main.rs

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use log::{info, error};
use std::env;

mod schedule_utils;

#[derive(Deserialize)]
struct GenerateScheduleRequest {
    courses: Vec<String>,
    minimum: usize,
}

#[derive(Serialize)]
struct GenerateScheduleResponse {
    schedule_groups: Vec<Vec<CourseSchedule>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CourseSchedule {
    id: i32,
    materia: Materia,
    profesor: Profesor,
    salon: Salon,
    id_del_curso: i32,
    ciclo: i32,
    sesion: String,
    mat_comb: i32,
    clases_comb: String,
    capacidad_inscripcion_combinacion: i32,
    no_de_catalogo: String,
    clase: String,
    no_de_clase: i32,
    capacidad_inscripcion: i32,
    total_inscripciones: i32,
    total_inscripciones_materia_combinada: i32,
    fecha_inicial: String,
    fecha_final: String,
    capacidad_del_salon: i32,
    hora_inicio: String,
    hora_fin: String,
    lunes: bool,
    martes: bool,
    miercoles: bool,
    jueves: bool,
    viernes: bool,
    sabado: bool,
    domingo: bool,
    bloque_optativo: String,
    idioma_impartido: Option<String>,
    modalidad_clase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Materia {
    id: i32,
    codigo: String,
    nombre: String,
    no_de_catalogo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Profesor {
    id: i32,
    nombre: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Salon {
    id: i32,
    nombre: String,
    capacidad: i32,
}
// Handler for generating schedules
async fn generate_schedule(req_body: web::Json<GenerateScheduleRequest>) -> impl Responder {
    // Fetch data from the Django API
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .expect("Failed to build HTTP client");

    // Replace with your actual Django API URL
    let django_api_url = env::var("DJANGO_API_URL").unwrap_or_else(|_| "http://127.0.0.1:8001/api/cursos/".to_string());

    let response = client.get(&django_api_url)
        .send()
        .await;

    let courses_data: Vec<CourseSchedule> = match response {
        Ok(resp) => {
            match resp.json().await {
                Ok(data) => data,
                Err(err) => {
                    error!("Error parsing response from Django API: {}", err);
                    return HttpResponse::InternalServerError()
                        .body(format!("Error parsing response from Django API: {}", err));
                }
            }
        },
        Err(err) => {
            error!("Error fetching data from Django API: {}", err);
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching data from Django API: {}", err));
        }
    };

    // Process the data to create compatible schedules
    let compatible_schedules = schedule_utils::create_compatible_schedules(
        &courses_data,
        &req_body.courses,
        req_body.minimum,
    );

    // Prepare the response
    let response = GenerateScheduleResponse {
        schedule_groups: compatible_schedules,
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Set the server port
    let port = 8082;

    // Start the HTTP server
    info!("Starting server at http://0.0.0.0:{}", port);

    // Attempt to bind and run the server, logging any errors
    let server = HttpServer::new(|| {
        App::new()
            .route("/generate_schedule", web::post().to(generate_schedule))
    })
    .bind(("0.0.0.0", port));

    match server {
        Ok(srv) => {
            if let Err(e) = srv.run().await {
                error!("Server encountered an error while running: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to bind to port {}: {}", port, e);
        }
    }
}
