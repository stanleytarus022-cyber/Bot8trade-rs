use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mt5_host: String,
    pub mt5_port: u16,
    pub mt5_username: String,
    pub mt5_password: String,
    pub api_key: String,
    pub database_url: String,
    pub news_api_key: String,
    pub update_interval_seconds: u64,
    pub min_confidence: f64,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Config {
            mt5_host: env::var("MT5_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            mt5_port: env::var("MT5_PORT")
                .unwrap_or_else(|_| "5005".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("Invalid MT5_PORT".to_string()))?,
            mt5_username: env::var("MT5_USERNAME")
                .map_err(|_| ConfigError::MissingEnv("MT5_USERNAME".to_string()))?,
            mt5_password: env::var("MT5_PASSWORD")
                .map_err(|_| ConfigError::MissingEnv("MT5_PASSWORD".to_string()))?,
            api_key: env::var("API_KEY")
                .map_err(|_| ConfigError::MissingEnv("API_KEY".to_string()))?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://bot8trade.db".to_string()),
            news_api_key: env::var("NEWS_API_KEY")
                .unwrap_or_else(|_| String::new()),
            update_interval_seconds: env::var("UPDATE_INTERVAL")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            min_confidence: env::var("MIN_CONFIDENCE")
                .unwrap_or_else(|_| "0.75".to_string())
                .parse()
                .unwrap_or(0.75),
        })
    }
}
