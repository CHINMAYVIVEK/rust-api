
use actix_web::{  HttpResponse,  Responder};

pub async fn health_check() -> impl Responder {
    println!("Health check");
    HttpResponse::Ok().body("Health check")
}

pub async fn login_handler() -> impl Responder {
    println!("Hello, world!");
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn registration_handler(req_body: String) -> impl Responder {
    println!("request: {}", req_body);
    HttpResponse::Ok().body(req_body)
}