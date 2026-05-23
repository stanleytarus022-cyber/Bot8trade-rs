use crate::config::Config;
use crate::models::MarketData;
use reqwest::Client;
use thiserror::Error;
use chrono::Utc;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self, ApiError> {
        Ok(ApiClient {
            client: Client::new(),
            config: config.clone(),
        })
    }

    pub async fn fetch_gold_price(&self) -> Result<MarketData, ApiError> {
        // This would integrate with real APIs like:
        // - Alpha Vantage
        // - IEX Cloud
        // - Finnhub
        // - MetaQuotes API
        
        let response = self.client
            .get("https://api.example.com/gold/price")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let data = response.json::<serde_json::Value>().await?;
        
        Ok(MarketData {
            timestamp: Utc::now(),
            symbol: "GOLD".to_string(),
            open: data["open"].as_f64().unwrap_or(0.0),
            high: data["high"].as_f64().unwrap_or(0.0),
            low: data["low"].as_f64().unwrap_or(0.0),
            close: data["close"].as_f64().unwrap_or(0.0),
            volume: data["volume"].as_u64().unwrap_or(0),
        })
    }

    pub async fn fetch_news(&self, query: &str) -> Result<Vec<String>, ApiError> {
        // Integrate with news APIs (NewsAPI, etc.)
        Ok(vec![])
    }
}
