// backwellApi/src/main.rs

use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize, Deserializer};
use reqwest::Client;
use log::{info, error};
use std::collections::HashSet;
use std::env;
use url::Url;

mod schedule_utils;

/// Structure representing the incoming schedule generation request.
#[derive(Deserialize)]
struct GenerateScheduleRequest {
    courses: Vec<String>, // List of course names requested
    minimum: usize,       // Minimum number of courses in a schedule
}

/// Structure representing the entire response payload.
#[derive(Serialize)]
struct GenerateScheduleResponse {
    data: ResponseData, // Encapsulates all response data
}

/// Structure containing detailed response data.
#[derive(Serialize)]
struct ResponseData {
    status: u16,                        // HTTP status code
    compatible_schedules: Vec<Vec<CourseSchedule>>, // Detailed compatible course schedules
    simplified_schedules: Vec<ScheduleGroup>,      // Simplified schedules for frontend
    messages: Vec<String>,              // Informational or error messages
}

/// Structure representing a group of courses in a schedule.
#[derive(Serialize)]
struct ScheduleGroup {
    schedule_number: usize,             // Identifier for the schedule group
    courses: Vec<CourseInfo>,           // List of courses in the group
}

/// Structure containing information about a single course.
#[derive(Serialize)]
struct CourseInfo {
    materia: String,                    // Name of the course
    profesor: String,                   // Name of the professor
    schedules: Vec<ScheduleInfo>,       // List of schedules for the course
}

/// Structure representing the schedule details of a course.
#[derive(Serialize)]
struct ScheduleInfo {
    dia: String,                        // Day of the week
    hora_inicio: String,                // Start time
    hora_fin: String,                   // End time
    salon: String,                      // Classroom name
}

/// Detailed structure representing a course schedule fetched from Django API.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CourseSchedule {
    id: i32,
    materia: Materia,
    #[serde(deserialize_with = "deserialize_profesor")]
    profesor: Option<Profesor>,
    #[serde(deserialize_with = "deserialize_profesor")]
    adjunto: Option<Profesor>,
    schedules: Vec<Schedule>,
    id_del_curso: String,
    ciclo: String,
    sesion: String,
    seccion_clase: Option<String>,
    grupo_academico: Option<String>,
    organizacion_academica: Option<String>,
    intercambio: Option<String>,
    inter_plantel: Option<String>,
    oficialidad_materia: Option<String>,
    plan_academico: Option<String>,
    sede: Option<String>,
    id_administrador_curso: Option<String>,
    nombre_administrador_curso: Option<String>,
    mat_comb: Option<String>,
    clases_comb: Option<String>,
    capacidad_inscripcion_combinacion: Option<i32>,
    no_de_catalogo: Option<String>,
    clase: Option<String>,
    no_de_clase: String,
    capacidad_inscripcion: i32,
    total_inscripciones: i32,
    total_inscripciones_materia_combinada: i32,
    fecha_inicial: Option<String>,
    fecha_final: Option<String>,
    bloque_optativo: Option<String>,
    idioma_impartido: Option<String>,
    modalidad_clase: Option<String>,
}

/// Structure representing a classroom.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Salon {
    id: i32,
    nombre: String,
    capacidad: Option<i32>,
}

/// Structure representing a subject/course.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Materia {
    id: i32,
    nombre: String,
    no_de_catalogo: Option<String>,
    codigo: Option<String>,
}

/// Structure representing a professor.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Profesor {
    id: i32,
    nombre: Option<String>,
    id_profesor: Option<String>,
}

/// Structure representing a schedule.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Schedule {
    id: i32,
    dia: String,
    hora_inicio: String,
    hora_fin: String,
    salon: Salon,
}

/// Custom deserializer for `Option<Profesor>` fields to handle various data formats.
fn deserialize_profesor<'de, D>(deserializer: D) -> Result<Option<Profesor>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ProfesorOrId {
        Null,                    // Represents `null` values
        Id(i32),                 // Represents an integer ID
        Object(Profesor),        // Represents a full `Profesor` object
    }

    match ProfesorOrId::deserialize(deserializer)? {
        ProfesorOrId::Null => Ok(None), // Maps `null` to `None`
        ProfesorOrId::Id(id) => Ok(Some(Profesor {
            id,
            nombre: None,
            id_profesor: None,
        })),
        ProfesorOrId::Object(profesor) => Ok(Some(profesor)),
    }
}

/// Asynchronous handler for generating schedules based on user requests.
async fn generate_schedule(req_body: web::Json<GenerateScheduleRequest>) -> impl actix_web::Responder {
    // Initialize the HTTP client
    let client = Client::builder()
        .build()
        .expect("Failed to build HTTP client");

    // Fetch the Django API base URL from environment variables or use the default
    let django_api_base_url = env::var("DJANGO_API_URL")
        .unwrap_or_else(|_| "http://web:8000/api/cursos/".to_string());

    let mut url = Url::parse(&django_api_base_url).expect("Invalid Django API URL");

    // Build query parameters to fetch only selected subjects
    if !req_body.courses.is_empty() {
        url.query_pairs_mut()
            .append_pair("materia__nombre__in", &req_body.courses.join(","));
    }

    // Send GET request to the Django API
    let response = client.get(url).send().await;

    // Parse the response into `CourseSchedule` structures
    let courses_data: Vec<CourseSchedule> = match response {
        Ok(resp) => {
            match resp.json().await {
                Ok(data) => data,
                Err(err) => {
                    error!("Error parsing response from Django API: {}", err);
                    return HttpResponse::InternalServerError()
                        .json(GenerateScheduleResponse {
                            data: ResponseData {
                                status: 500,
                                compatible_schedules: vec![],
                                simplified_schedules: vec![],
                                messages: vec![format!("Error parsing response from Django API: {}", err)],
                            }
                        });
                }
            }
        },
        Err(err) => {
            error!("Error fetching data from Django API: {}", err);
            return HttpResponse::InternalServerError()
                .json(GenerateScheduleResponse {
                    data: ResponseData {
                        status: 500,
                        compatible_schedules: vec![],
                        simplified_schedules: vec![],
                        messages: vec![format!("Error fetching data from Django API: {}", err)],
                    }
                });
        }
    };

    // Determine which subjects were found and which were not
    let subjects_found: HashSet<String> = courses_data.iter()
        .map(|course| course.materia.nombre.trim().to_string())
        .collect();
    let subjects_requested: HashSet<String> = req_body.courses.iter()
        .map(|s| s.trim().to_string())
        .collect();

    let subjects_not_found: Vec<String> = subjects_requested.difference(&subjects_found).cloned().collect();
    let mut messages = Vec::new();

    if !subjects_not_found.is_empty() {
        messages.push(format!("Subjects not found: {}", subjects_not_found.join(", ")));
    }

    // Adjust the minimum parameter if it exceeds the number of available courses
    let num_courses_available = courses_data.len();
    let adjusted_minimum = if req_body.minimum > num_courses_available {
        num_courses_available
    } else {
        req_body.minimum
    };

    if req_body.minimum > num_courses_available {
        messages.push(format!(
            "Adjusted minimum from {} to {} due to limited available courses.",
            req_body.minimum, adjusted_minimum
        ));
    }

    // Generate compatible schedules using the optimized algorithm
    let compatible_schedules = schedule_utils::create_compatible_schedules(
        &courses_data,
        &req_body.courses,
        adjusted_minimum,
    );

    let mut final_schedules = compatible_schedules.clone();

    // Handle cases where no compatible schedules are found
    if final_schedules.is_empty() {
        if adjusted_minimum <= 1 {
            for course in &courses_data {
                final_schedules.push(vec![course.clone()]);
            }
            messages.push("No combinations possible, showing individual courses.".to_string());
        } else {
            messages.push("No combinations possible with the requested minimum.".to_string());
        }
    }

    // Simplify schedules for frontend consumption
    let simplified_schedules = simplify_schedules(&final_schedules);

    // Construct the optimized response
    let response = GenerateScheduleResponse {
        data: ResponseData {
            status: 200,
            compatible_schedules: final_schedules,
            simplified_schedules,
            messages,
        },
    };

    HttpResponse::Ok().json(response)
}

/// Function to simplify detailed schedules into a frontend-friendly format.
fn simplify_schedules(schedules: &Vec<Vec<CourseSchedule>>) -> Vec<ScheduleGroup> {
    let mut result = Vec::new();

    for (index, schedule_group) in schedules.iter().enumerate() {
        let mut courses_info = Vec::new();

        for course in schedule_group {
            let mut schedules_info = Vec::new();
            for sched in &course.schedules {
                schedules_info.push(ScheduleInfo {
                    dia: sched.dia.clone(),
                    hora_inicio: sched.hora_inicio.clone(),
                    hora_fin: sched.hora_fin.clone(),
                    salon: sched.salon.nombre.clone(),
                });
            }
            courses_info.push(CourseInfo {
                materia: course.materia.nombre.clone(),
                profesor: course.profesor.as_ref()
                    .and_then(|p| p.nombre.clone())
                    .unwrap_or_else(|| "Unknown".to_string()), // Default to "Unknown" if profesor is None
                schedules: schedules_info,
            });
        }

        result.push(ScheduleGroup {
            schedule_number: index + 1,
            courses: courses_info,
        });
    }

    result
}

/// Entry point of the Actix-web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    println!("Server is starting...");
    let port = 8082;
    info!("Starting server at http://0.0.0.0:{}", port);

    // Configure the Actix-web server
    HttpServer::new(|| {
        App::new()
            .route("/v1/api/generate_schedule", web::post().to(generate_schedule))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
