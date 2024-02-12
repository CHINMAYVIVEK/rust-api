use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;


use dotenv::dotenv;

use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    // Add other configuration parameters as needed
}

impl AppConfig {
    pub fn load() -> Result<Self, dotenv::Error> {
        dotenv().ok();

        let db_name = env::var("POSTGRESQL_DB_NAME").map_err(|err| dotenv::Error::EnvVar(err))?;
        let db_user = env::var("POSTGRESQL_DB_USER").map_err(|err| dotenv::Error::EnvVar(err))?;
        let db_password = env::var("POSTGRESQL_DB_PASSWORD").map_err(|err| dotenv::Error::EnvVar(err))?;
        let db_port = env::var("POSTGRESQL_DB_PORT").map_err(|err| dotenv::Error::EnvVar(err))?;
        let db_ip = env::var("POSTGRESQL_DB_IP").map_err(|err| dotenv::Error::EnvVar(err))?;
       
        println!("postgresql://{}:{}@{}:{}/{}",db_user, db_password, db_ip, db_port, db_name);
        
        Ok(AppConfig {
            database_url: format!(
                "postgresql://{}:{}@{}:{}/{}",
                db_user, db_password, db_ip, db_port, db_name
            ),
        })
    }
}

pub async fn connect_to_database(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}
