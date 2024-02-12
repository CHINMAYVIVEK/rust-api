
mod config;
mod server;


use config::postgres::AppConfig;
use server::router;

fn main() {
    

    // Load configuration
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            std::process::exit(1);
        }
    };

    // Connect to database
    // let _pool = match config::postgres::connect_to_database(&config).await {
    //     Ok(pool) => pool,
    //     Err(err) => {
    //         eprintln!("Error connecting to database: {}", err);
    //         std::process::exit(1);
    //     }
    // };
    let _= config::postgres::connect_to_database(&config);

    let _ = router::start_server();
}




