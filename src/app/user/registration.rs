// src/app/user/registration.rs

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Error};
use bcrypt::hash;

#[derive(Debug, Deserialize)]
pub struct RegistrationRequest {
    first_name: String,
    last_name: String,
    gender: String,
    age: u32,
    email: String,
    phone_number: String,
    salary: u32,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationResponse {
    code: u16,
    status: String,
    message: String,
}

pub async fn register(pool: web::Data<PgPool>, req_body: web::Json<RegistrationRequest>) -> Result<HttpResponse, Error> {
    let RegistrationRequest { first_name, last_name, gender, age, email, phone_number, salary, password } = req_body.into_inner();

    let hashed_password = hash(&password, DEFAULT_COST)?;
    
    sqlx::query!(
        r#"
        INSERT INTO users (first_name, last_name, gender, age, email, phone_number, salary, password_hash)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        first_name, last_name, gender, age, email, phone_number, salary, hashed_password
    )
    .execute(pool.get_ref())
    .await?;

    let response = RegistrationResponse {
        code: 200,
        status: "OK".to_string(),
        message: "Registration successful".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}
