// server/handler.rs

use actix_web::{ HttpResponse, Responder,http::StatusCode};
use serde_json::json;


pub async fn health_check() -> impl Responder {
    println!("Health check");
    let response_body = json!({
        "code": StatusCode::OK.as_u16(),
        "status": StatusCode::OK.canonical_reason().unwrap_or("UNKNOWN"),
        "message": "Health check"
    });

    HttpResponse::Ok().json(response_body)
}




pub async fn registration_handler(req_body: String) -> impl Responder {
    println!("request: {}", req_body);
    HttpResponse::Ok().json(req_body)
}