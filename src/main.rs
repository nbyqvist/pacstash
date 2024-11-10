mod cached_package;
mod config;
mod disk_cache;
mod error;
mod fetch;
mod handler;
mod response;
mod state;
mod statistics;
mod upstream;
mod upstream_mirror;

use std::sync::Arc;

use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use config::Config;
use handler::{caching_package_endpoint, statistics_page};
use sqlx::sqlite::SqlitePoolOptions;
use state::AppStateStruct;

#[get("/")]
async fn stub() -> impl Responder {
    "Pacstash, this will return stats eventually".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::load()?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
            .service(statistics_page)
            .service(caching_package_endpoint)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(Arc::new(AppStateStruct{cache_root: cfg.cache_root.clone()})))
    })
    .bind((cfg.web_host, cfg.web_port))
    .expect("Could not bind address")
    .run()
    .await
    .expect("Failed to startup application");

    Ok(())
}
