// server/router.rs

use super::handler::{registration_handler, health_check, login_handler};
use actix_web::{web, App, HttpServer};

use dotenv::dotenv;
use std::env;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    dotenv().ok();
    let server_ip = env::var("SERVER_IP").expect("SERVER_IP not set in .env");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT not set in .env");

    let server_address = format!("{}:{}", server_ip, server_port);
    println!("Server : {}", server_address);

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/home", web::get().to(login_handler))
            .route("/echo", web::post().to(registration_handler))
    })
    .bind(server_address)?
    .run()
    .await
}
