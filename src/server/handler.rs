// server/handler.rs

use actix_web::{web, HttpResponse, Responder,http::StatusCode};
use serde_json::json;
use sqlx::PgPool;

use crate::app::user::login::{self, ResponseData};
use crate::app::user::registration;

pub async fn login_handler(req_body: web::Json<login::LoginRequest>, pool: web::Data<PgPool>) -> impl Responder {
    match login::login(pool.get_ref(), req_body.into_inner()).await {
        Ok(response_data) => {
            let status_code = StatusCode::from_u16(response_data.code).unwrap();
            HttpResponse::build(status_code)
                .json(response_data)
        }
        Err(_) => {
            let response_data = ResponseData::new(StatusCode::INTERNAL_SERVER_ERROR, "Error during login");
            HttpResponse::InternalServerError().json(response_data)
        }
    }
}


pub async fn registration_handler(req_body: web::Json<registration::RegistrationRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    match registration::register(pool, req_body).await {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().json(registration::RegistrationResponse {
            code: 500,
            status: "Internal Server Error".to_string(),
            message: "Failed to register user".to_string(),
        }),
    }
}

pub async fn health_check() -> impl Responder {
    println!("Health check");
    let response_body = json!({
        "code": StatusCode::OK.as_u16(),
        "status": StatusCode::OK.canonical_reason().unwrap_or("UNKNOWN"),
        "message": "Health check"
    });

    HttpResponse::Ok().json(response_body)
}