#![feature(type_alias_impl_trait)]

mod cached_package;
mod config;
mod disk_cache;
mod error;
mod fetch;
mod routes;
mod state;
mod statistics;
mod templates;
mod upstream;
mod upstream_mirror;

use std::sync::Arc;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use config::Config;
use routes::{caching_package_endpoint, get_mirrors_for_upstream, not_found_page, purge_expired_packages, statistics_page, view_repo_page};
use sqlx::sqlite::SqlitePoolOptions;
use state::AppStateStruct;

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
            .service(view_repo_page)
            .service(caching_package_endpoint)
            .service(purge_expired_packages)
            .service(get_mirrors_for_upstream)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(Arc::new(AppStateStruct {
                cache_root: cfg.cache_root.clone(),
                pkg_max_age: cfg.pkg_max_age,
            })))
            .default_service(actix_web::web::route().to(not_found_page))
    })
    .bind((cfg.web_host, cfg.web_port))
    .expect("Could not bind address")
    .run()
    .await
    .expect("Failed to startup application");

    Ok(())
}
