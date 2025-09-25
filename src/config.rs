use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

fn default_database_url() -> String {
    "sqlite:./bbs.db".to_string()
}

fn default_port() -> String {
    "8080".to_string()
}

fn default_rust_log() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "DATABASE_URL", default = "default_database_url")]
    pub database_url: String,

    #[serde(rename = "PORT", default = "default_port")]
    pub port: String,

    #[serde(rename = "RUST_LOG", default = "default_rust_log")]
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = ConfigBuilder::builder()
            .add_source(
                File::with_name(".env")
                    .format(config::FileFormat::Ini)
                    .required(false)
            )
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }

    // pub fn validate(&self) -> Result<(), String> {
    //     if self.database_url.is_empty() {
    //         return Err("DATABASE_URL cannot be empty".to_string());
    //     }
    // 
    //     if self.port.parse::<u16>().is_err() {
    //         return Err("PORT must be a valid port number".to_string());
    //     }
    // 
    //     if self.rust_log.is_empty() {
    //         return Err("RUST_LOG cannot be empty".to_string());
    //     }
    // 
    //     Ok(())
    // }
}