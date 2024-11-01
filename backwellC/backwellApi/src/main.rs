use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use log::{info, error};
use std::collections::HashMap;
use std::env;
use url::Url;

mod schedule_utils;

#[derive(Deserialize)]
struct GenerateScheduleRequest {
    courses: Vec<String>,
    minimum: usize,
}

#[derive(Serialize)]
struct GenerateScheduleResponse {
    response: u16,
    data: Vec<Vec<CourseSchedule>>,
    schedule_s: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CourseSchedule {
    id: i32,
    materia: Materia,
    profesor: Profesor,
    schedules: Vec<Schedule>,
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
    bloque_optativo: String,
    idioma_impartido: Option<String>,
    modalidad_clase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Schedule {
    id: i32,
    salon: Salon,
    profesor: Profesor,
    dia: String,
    hora_inicio: String,
    hora_fin: String,
    curso: i32,
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
    nombre: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Salon {
    id: i32,
    nombre: String,
    capacidad: Option<i32>,
}

async fn generate_schedule(req_body: web::Json<GenerateScheduleRequest>) -> impl actix_web::Responder {
    let client = Client::builder()
        .build()
        .expect("Failed to build HTTP client");

    let django_api_base_url = env::var("DJANGO_API_URL")
        .unwrap_or_else(|_| "http://web:8000/api/cursos/".to_string());

    let mut url = Url::parse(&django_api_base_url).expect("Invalid Django API URL");
    for course_name in &req_body.courses {
        url.query_pairs_mut()
            .append_pair("materia__nombre", course_name);
    }

    let response = client.get(url)
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

    let compatible_schedules = schedule_utils::create_compatible_schedules(
        &courses_data,
        &req_body.courses,
        req_body.minimum,
    );

    let schedule_s = simplify_schedules(&compatible_schedules);

    let response = GenerateScheduleResponse {
        response: 200,
        data: compatible_schedules,
        schedule_s,
    };

    HttpResponse::Ok().json(response)
}

// Simplifies the schedule for the `scheduleS` section
fn simplify_schedules(schedules: &Vec<Vec<CourseSchedule>>) -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    let mut result = HashMap::new();

    for (i, schedule_group) in schedules.iter().enumerate() {
        let mut schedule_map = HashMap::new();
        for course in schedule_group {
            let mut days_map = HashMap::new();
            for sched in &course.schedules {
                let time_range = format!("{} - {}", sched.hora_inicio, sched.hora_fin);
                days_map.insert(sched.dia.clone(), time_range);
            }
            schedule_map.insert(course.materia.nombre.clone(), days_map);
        }
        result.insert(format!("horario{}", i + 1), schedule_map);
    }

    result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port = 8082;
    info!("Iniciando servidor en http://0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .route("/v1/api/generate", web::post().to(generate_schedule))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    info!("Servidor Actix finalizado"); // Debe mantenerse activo
    Ok(())
}
