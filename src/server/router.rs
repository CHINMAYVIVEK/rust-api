// src/server.rs

use super::handler::{registration_handler, health_check, login_handler};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/home", web::get().to(login_handler))
            .route("/echo", web::post().to(registration_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
