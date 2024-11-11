use std::env::var;

pub struct Config {
    pub database_url: String,
    pub cache_root: String,
    pub web_host: String,
    pub web_port: u16,
    pub pkg_max_age: u32,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let database_url = var("DATABASE_URL")?;
        let cache_root = var("CACHE_ROOT")?;
        let web_host = var("WEB_HOST")?;
        let web_port_str = var("WEB_PORT")?;
        let web_port = web_port_str.parse::<u16>()?;
        let pkg_max_age_str = var("PACKAGE_MAX_AGE_SECONDS")?;
        let pkg_max_age = pkg_max_age_str.parse::<u32>()?;

        log::info!("Using database: {database_url}");
        log::info!("Using cache folder: {cache_root}");
        log::info!("Using address {web_host}:{web_port}");
        log::info!("Using package_max_age {pkg_max_age}");

        Ok(Self {
            database_url,
            cache_root,
            web_host,
            web_port,
            pkg_max_age,
        })
    }
}