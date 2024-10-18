use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{User, NewUser};
use crate::schema::users::dsl::*;
use crate::DbPool;
use uuid::Uuid;
use bcrypt::{hash, verify};

pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");

    let result = web::block(move || {
        users.load::<User>(&conn)
    }).await;

    match result {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(e) => {
            eprintln!("Error al obtener usuarios: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let user_id = user_id.into_inner();

    let result = web::block(move || {
        users.filter(id.eq(user_id)).first::<User>(&conn)
    }).await;

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
        diesel::insert_into(users).values(&user).get_result::<User>(&conn)
    }).await;

    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Error al crear usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>, updated_user: web::Json<NewUser>) -> impl Responder {
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
    }).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Error al actualizar usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let user_id = user_id.into_inner();

    let result = web::block(move || {
        diesel::delete(users.filter(id.eq(user_id))).execute(&conn)
    }).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario eliminado."),
        Err(e) => {
            eprintln!("Error al eliminar usuario: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::User;
use crate::schema::users::dsl::*;
use crate::DbPool;
use crate::utils::auth::create_jwt;
use serde::Deserialize;
use bcrypt::verify;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub async fn login(pool: web::Data<DbPool>, info: web::Json<LoginInfo>) -> impl Responder {
    let conn = pool.get().expect("No se pudo obtener la conexión del pool");
    let login_info = info.into_inner();

    let result = web::block(move || {
        users.filter(username.eq(login_info.username)).first::<User>(&conn)
    }).await;

    match result {
        Ok(user) => {
            if verify(&login_info.password, &user.password).unwrap_or(false) {
                if let Ok(token) = create_jwt(&user.id.to_string()) {
                    return HttpResponse::Ok().json(serde_json::json!({ "token": token }));
                } else {
                    return HttpResponse::InternalServerError().body("Error al crear token");
                }
            } else {
                return HttpResponse::Unauthorized().body("Credenciales incorrectas");
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Credenciales incorrectas"),
    }
}
