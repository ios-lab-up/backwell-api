// src/handlers/user_handlers.rs

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
use bcrypt::{hash, verify};
use serde::Deserialize;

use crate::models::{NewUser, User};
use crate::utils::auth::{create_jwt};
use crate::utils::auth_middleware::AuthorizedUser;

// Importar la tabla users desde models::user
use crate::models::user::users::dsl::*;

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub async fn login(pool: web::Data<DbPool>, info: web::Json<LoginInfo>) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let login_info = info.into_inner();

    let result = web::block(move || {
        users
            .filter(username.eq(login_info.username))
            .first::<User>(&conn)
    })
    .await;

    match result {
        Ok(user) => {
            if verify(&login_info.password, &user.password).unwrap_or(false) {
                match create_jwt(&user.id.to_string()) {
                    Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
                    Err(e) => {
                        eprintln!("Error al crear token: {:?}", e);
                        HttpResponse::InternalServerError().body("Error al crear token")
                    }
                }
            } else {
                HttpResponse::Unauthorized().body("Credenciales incorrectas")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Credenciales incorrectas"),
    }
}

pub async fn get_users(pool: web::Data<DbPool>, _user: AuthorizedUser) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");

    let result = web::block(move || users.load::<User>(&conn)).await;

    match result {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(e) => {
            eprintln!("Error al obtener usuarios: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    _user: AuthorizedUser,
) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let user_id = user_id.into_inner();

    let result = web::block(move || users.filter(id.eq(user_id)).first::<User>(&conn)).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Usuario no encontrado"),
        Err(e) => {
            eprintln!("Error al obtener usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let mut user = new_user.into_inner();

    // Hashear la contraseña
    match hash(&user.password, 4) {
        Ok(hashed_password) => user.password = hashed_password,
        Err(e) => {
            eprintln!("Error al hashear contraseña: {:?}", e);
            return HttpResponse::InternalServerError().body("Error al crear usuario");
        }
    }

    let result = web::block(move || {
        diesel::insert_into(users)
            .values(&user)
            .get_result::<User>(&conn)
    })
    .await;

    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Error al crear usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    updated_user: web::Json<NewUser>,
    _user: AuthorizedUser,
) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let user_id = user_id.into_inner();
    let updated_user = updated_user.into_inner();

    let result = web::block(move || {
        diesel::update(users.filter(id.eq(user_id)))
            .set((
                name.eq(updated_user.name),
                email.eq(updated_user.email),
                username.eq(updated_user.username),
                password.eq(updated_user.password),
                is_staff.eq(updated_user.is_staff),
            ))
            .get_result::<User>(&conn)
    })
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Error al actualizar usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    _user: AuthorizedUser,
) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let user_id = user_id.into_inner();

    let result = web::block(move || diesel::delete(users.filter(id.eq(user_id))).execute(&conn)).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario eliminado."),
        Err(e) => {
            eprintln!("Error al eliminar usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}