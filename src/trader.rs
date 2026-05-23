use crate::api::ApiClient;
use crate::analyzer::MarketAnalyzer;
use crate::database::Database;
use crate::mt5::MT5Connection;
use crate::models::PredictionDirection;
use crate::predictor::GoldPredictor;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use log::{info, error};

pub async fn start_trading_loop(
    api_client: ApiClient,
    analyzer: MarketAnalyzer,
    predictor: GoldPredictor,
    mt5: MT5Connection,
    db: Arc<RwLock<Database>>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting main trading loop...");

    loop {
        match run_trading_cycle(&api_client, &analyzer, &predictor, &mt5, &db).await {
            Ok(_) => {
                info!("Trading cycle completed successfully");
            }
            Err(e) => {
                error!("Error in trading cycle: {}", e);
            }
        }

        // Wait before next cycle
        sleep(Duration::from_secs(300)).await; // 5 minutes
    }
}

async fn run_trading_cycle(
    api_client: &ApiClient,
    analyzer: &MarketAnalyzer,
    predictor: &GoldPredictor,
    mt5: &MT5Connection,
    db: &Arc<RwLock<Database>>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running trading cycle...");

    // 1. Fetch current market data
    let market_data = api_client.fetch_gold_price().await?;
    info!("Fetched market data: {}", market_data.close);

    // 2. Get historical data for analysis
    let db_lock = db.read().await;
    let historical_data = db_lock
        .get_recent_market_data(&market_data.symbol, 200)
        .await?;
    drop(db_lock);

    // 3. Analyze technical indicators
    let mut all_data = historical_data.clone();
    all_data.push(market_data.clone());
    let indicators = analyzer.analyze(&all_data);

    // 4. Fetch news sentiment
    let news = api_client.fetch_news("gold price").await?;
    let sentiment = calculate_sentiment(&news);

    // 5. Generate prediction
    let prediction = predictor.predict(&market_data, &indicators, sentiment);

    // 6. Save prediction and market data
    let mut db_lock = db.write().await;
    db_lock.save_market_data(&market_data).await?;
    db_lock.save_prediction(&prediction).await?;
    drop(db_lock);

    // 7. Execute trade if confidence is high enough
    if prediction.confidence > 0.75 && prediction.direction != PredictionDirection::Hold {
        match mt5.execute_trade(&prediction).await {
            Ok(trade) => {
                info!("Trade executed: {:?}", trade);
                let mut db_lock = db.write().await;
                db_lock.save_trade(&trade).await?;
            }
            Err(e) => {
                error!("Failed to execute trade: {}", e);
            }
        }
    }

    Ok(())
}

fn calculate_sentiment(news_items: &[String]) -> f64 {
    // TODO: Implement sentiment analysis using NLP
    // For now, return neutral sentiment
    0.0
}
