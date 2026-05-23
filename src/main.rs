mod config;
mod api;
mod analyzer;
mod predictor;
mod trader;
mod database;
mod mt5;
mod models;
mod utils;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    env_logger::init();
    
    info!("Starting Bot8trade-rs...");
    
    // Load configuration
    let config = config::Config::from_env()?;
    info!("Configuration loaded successfully");
    
    // Initialize database
    let db = Arc::new(RwLock::new(database::Database::new(&config).await?));
    info!("Database initialized");
    
    // Initialize API client
    let api_client = api::ApiClient::new(&config)?;
    info!("API client initialized");
    
    // Initialize analyzer
    let analyzer = analyzer::MarketAnalyzer::new();
    
    // Initialize predictor
    let predictor = predictor::GoldPredictor::new();
    
    // Initialize MetaTrader5 connection
    let mt5 = mt5::MT5Connection::new(&config).await?;
    info!("MetaTrader5 connection established");
    
    // Start the main trading loop
    trader::start_trading_loop(api_client, analyzer, predictor, mt5, db).await?;
    
    Ok(())
}
