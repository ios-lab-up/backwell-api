// backwellApi/src/main.rs

use actix_web::{web, App, HttpResponse, HttpServer};
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashSet;
use std::env;
use url::Url;

mod schedule_utils;

#[derive(Deserialize)]
struct GenerateScheduleRequest {
    courses: Vec<String>,
    minimum: usize,
    professors: Option<Vec<String>>,
}

#[derive(Serialize)]
struct GenerateScheduleResponse {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    status: u16,
    compatible_schedules: Vec<Vec<CourseSchedule>>,
    simplified_schedules: Vec<ScheduleGroup>,
    messages: Vec<String>,
}

#[derive(Serialize)]
struct ScheduleGroup {
    schedule_number: usize,
    courses: Vec<CourseInfo>,
}

#[derive(Serialize)]
struct CourseInfo {
    materia: String,
    profesor: String,
    schedules: Vec<ScheduleInfo>,
}

#[derive(Serialize)]
struct ScheduleInfo {
    dia: String,
    hora_inicio: String,
    hora_fin: String,
    salon: String,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Schedule {
    id: i32,
    dia: String,
    hora_inicio: String,
    hora_fin: String,
    salon: Salon,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Materia {
    id: i32,
    nombre: String,
    no_de_catalogo: Option<String>,
    codigo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Profesor {
    id: i32,
    nombre: Option<String>,
    id_profesor: Option<String>,
}

fn deserialize_profesor<'de, D>(deserializer: D) -> Result<Option<Profesor>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ProfesorOrId {
        Null,
        Id(i32),
        Object(Profesor),
    }
    match ProfesorOrId::deserialize(deserializer)? {
        ProfesorOrId::Null => Ok(None),
        ProfesorOrId::Id(id) => Ok(Some(Profesor {
            id,
            nombre: None,
            id_profesor: None,
        })),
        ProfesorOrId::Object(profesor) => Ok(Some(profesor)),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Salon {
    id: i32,
    nombre: String,
    capacidad: Option<i32>,
}

async fn generate_schedule(req_body: web::Json<GenerateScheduleRequest>) -> impl actix_web::Responder {
    let client = Client::builder().build().expect("Failed to build HTTP client");
    let django_api_base_url =
        env::var("DJANGO_API_URL").unwrap_or_else(|_| "http://web:8000/api/cursos/".to_string());
    let mut url = Url::parse(&django_api_base_url).expect("Invalid Django API URL");

    if !req_body.courses.is_empty() {
        url.query_pairs_mut()
            .append_pair("materia__nombre__in", &req_body.courses.join(","));
    }

    let mut messages = Vec::new();
    if req_body.minimum > req_body.courses.len() {
        messages.push(format!(
            "Minimum number of courses ({}) cannot be greater than the number of requested courses ({}).",
            req_body.minimum, req_body.courses.len()
        ));
        return HttpResponse::BadRequest().json(GenerateScheduleResponse {
            data: ResponseData {
                status: 400,
                compatible_schedules: vec![],
                simplified_schedules: vec![],
                messages,
            },
        });
    }

    let response = client.get(url).send().await;
    let courses_data: Vec<CourseSchedule> = match response {
        Ok(resp) => match resp.json().await {
            Ok(data) => data,
            Err(err) => {
                error!("Error parsing response from Django API: {}", err);
                return HttpResponse::InternalServerError().json(GenerateScheduleResponse {
                    data: ResponseData {
                        status: 500,
                        compatible_schedules: vec![],
                        simplified_schedules: vec![],
                        messages: vec![format!("Error parsing response from Django API: {}", err)],
                    },
                });
            }
        },
        Err(err) => {
            error!("Error fetching data from Django API: {}", err);
            return HttpResponse::InternalServerError().json(GenerateScheduleResponse {
                data: ResponseData {
                    status: 500,
                    compatible_schedules: vec![],
                    simplified_schedules: vec![],
                    messages: vec![format!("Error fetching data from Django API: {}", err)],
                },
            });
        }
    };

    let subjects_found: HashSet<String> = courses_data
        .iter()
        .map(|course| course.materia.nombre.trim().to_string())
        .collect();
    let subjects_requested: HashSet<String> =
        req_body.courses.iter().map(|s| s.trim().to_string()).collect();

    let subjects_not_found: Vec<String> = subjects_requested
        .difference(&subjects_found)
        .cloned()
        .collect();

    if !subjects_not_found.is_empty() {
        messages.push(format!(
            "Subjects not found: {}",
            subjects_not_found.join(", ")
        ));
    }

    let compatible_schedules = schedule_utils::create_compatible_schedules(
        &courses_data,
        &req_body.courses,
        &req_body.professors,
        req_body.minimum,
    );

    let final_schedules = compatible_schedules.clone();

    if final_schedules.is_empty() {
        if let Some(professors_list) = &req_body.professors {
            let professors_in_data: HashSet<_> = courses_data
                .iter()
                .filter_map(|course| {
                    course
                        .profesor
                        .as_ref()
                        .and_then(|p| p.nombre.as_ref())
                        .map(|name| name.trim().to_string())
                })
                .collect();
            let professors_list_trimmed: HashSet<_> =
                professors_list.iter().map(|p| p.trim().to_string()).collect();
            let missing_professors: Vec<_> = professors_list_trimmed
                .difference(&professors_in_data)
                .cloned()
                .collect();
            if !missing_professors.is_empty() {
                messages.push(format!(
                    "Professors not found teaching the selected courses: {}",
                    missing_professors.join(", ")
                ));
            } else {
                messages.push("No combinations possible with the given criteria.".to_string());
            }
        } else {
            messages.push("No combinations possible with the given criteria.".to_string());
        }
    }

    let simplified_schedules = simplify_schedules(&final_schedules);

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
                profesor: course
                    .profesor
                    .as_ref()
                    .and_then(|p| p.nombre.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port = 8082;
    info!("Starting server at http://0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new().route("/v1/api/generate_schedule", web::post().to(generate_schedule))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
