mod config;
mod db;
mod errors;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use r2d2_sqlite::SqliteConnectionManager;

use crate::config::Config;
use crate::handlers::trade;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config = Config::from_env().expect("Server configuration");

    // Create SQLite connection manager
    let manager = SqliteConnectionManager::file(&config.database_url);

    // Create connection pool
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool");

    // Initialize the database
    let conn = pool.get().expect("Failed to get db connection");
    db::init_db(&conn).expect("Database initialization failed");

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .expect("Failed to enable foreign keys");

    // Check if foreign keys are enabled
    if let Ok(enabled) = db::check_foreign_keys(&conn) {
        println!(
            "Foreign keys are {}",
            if enabled { "enabled" } else { "disabled" }
        );
    } else {
        eprintln!("Failed to check foreign key status");
    }

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .route("/trade", web::post().to(trade::add_or_update_trade))
                .route(
                    "/new-trades",
                    web::post().to(trade::get_new_trades_for_slave),
                )
                .route("/trade/close", web::post().to(trade::close_trade))
                .route("/trade/update-tpsl", web::post().to(trade::update_tpsl)),
        )
    })
    .bind(&config.server_addr)?
    .run()
    .await
}
