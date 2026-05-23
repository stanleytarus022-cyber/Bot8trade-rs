use crate::config::Config;
use crate::models::{MarketData, Trade, Prediction};
use thiserror::Error;
use log::info;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Error(String),
}

pub struct Database {
    config: Config,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, DatabaseError> {
        info!("Initializing database at {}", config.database_url);
        
        let db = Database {
            config: config.clone(),
        };

        // TODO: Initialize SQLx connection pool and run migrations
        // db.run_migrations().await?;
        
        Ok(db)
    }

    pub async fn save_market_data(&self, data: &MarketData) -> Result<(), DatabaseError> {
        // TODO: Insert into market_data table
        Ok(())
    }

    pub async fn save_prediction(&self, prediction: &Prediction) -> Result<(), DatabaseError> {
        // TODO: Insert into predictions table
        Ok(())
    }

    pub async fn save_trade(&self, trade: &Trade) -> Result<(), DatabaseError> {
        // TODO: Insert into trades table
        Ok(())
    }

    pub async fn get_recent_market_data(
        &self,
        symbol: &str,
        limit: i64,
    ) -> Result<Vec<MarketData>, DatabaseError> {
        // TODO: Query market_data table
        Ok(vec![])
    }
}
