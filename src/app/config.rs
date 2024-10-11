use serde::Deserialize;
use config::{Config, ConfigError};

/// Definition of the database configuration
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

/// Definition of the apps configuration
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

/// This method loads the config file from the application dir.
pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::builder();
    cfg = cfg.add_source(config::File::with_name("config.toml"));
    cfg.build()?.try_deserialize()
}