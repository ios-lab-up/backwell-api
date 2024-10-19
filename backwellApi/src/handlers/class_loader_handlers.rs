// src/handlers/class_loader_handlers.rs

use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use crate::utils::class_loader_utils::process_excel_data;
use diesel::prelude::*;
use diesel::SelectableHelper; // For `as_select` method
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Make sure the function is public
pub async fn load_course_list(
    mut payload: Multipart,
    pool: web::Data<DbPool>,
) -> impl Responder {
    // Create a temporary file to store the uploaded file
    let filepath = "./temp.xlsx";
    let mut f = web::block(move || File::create(filepath)).await.unwrap().unwrap();

    // Process multipart/form-data
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        let content_disposition = field.content_disposition().clone();

        if let Some(name) = content_disposition.get_name() {
            if name == "file" {
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || {
                        f.write_all(&data)?;
                        Ok::<_, std::io::Error>(f)
                    })
                    .await
                    .unwrap()
                    .unwrap();
                }
            }
        }
    }

    // Process the Excel file
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    if let Err(e) = process_excel_data(Path::new(filepath), &mut conn) {
        eprintln!("Error processing Excel file: {:?}", e);
        return HttpResponse::InternalServerError().body("Error processing file");
    }

    // Delete the temporary file
    std::fs::remove_file(filepath).unwrap();

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "data": "File processed successfully."}))
}

// Ensure other functions are also public if they are used elsewhere
pub async fn get_schedules(pool: web::Data<DbPool>) -> impl Responder {
    use crate::models::Schedule;
    use crate::schema::schedules::dsl::*;
    use diesel::SelectableHelper;

    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let result = web::block(move || {
        schedules
            .select(Schedule::as_select())
            .load::<Schedule>(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(schedule_list)) => HttpResponse::Ok().json(schedule_list),
        Ok(Err(e)) => {
            eprintln!("Error loading schedules: {:?}", e);
            HttpResponse::InternalServerError().body("Error retrieving schedules")
        }
        Err(e) => {
            eprintln!("Error executing web::block: {:?}", e);
            HttpResponse::InternalServerError().body("Error retrieving schedules")
        }
    }
}
