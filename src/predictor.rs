use crate::models::{Prediction, PredictionDirection, MarketData, TechnicalIndicators};
use chrono::Utc;
use log::info;

pub struct GoldPredictor {
    model: Option<String>, // Placeholder for ML model
}

impl GoldPredictor {
    pub fn new() -> Self {
        GoldPredictor { model: None }
    }

    pub fn predict(
        &self,
        market_data: &MarketData,
        indicators: &TechnicalIndicators,
        recent_sentiment: f64,
    ) -> Prediction {
        let direction = self.determine_direction(indicators, recent_sentiment);
        let confidence = self.calculate_confidence(indicators, recent_sentiment);
        let (target_price, stop_loss, take_profit) =
            self.calculate_targets(market_data, direction, indicators.atr);

        info!(
            "Prediction: {:?} with confidence: {:.2}%",
            direction,
            confidence * 100.0
        );

        Prediction {
            timestamp: Utc::now(),
            symbol: market_data.symbol.clone(),
            direction,
            confidence,
            target_price,
            stop_loss,
            take_profit,
        }
    }

    fn determine_direction(
        &self,
        indicators: &TechnicalIndicators,
        sentiment: f64,
    ) -> PredictionDirection {
        let rsi_signal = if indicators.rsi < 30.0 {
            1.0 // Oversold, bullish
        } else if indicators.rsi > 70.0 {
            -1.0 // Overbought, bearish
        } else {
            0.0
        };

        let sma_signal = if indicators.sma_50 > indicators.sma_200 {
            0.5 // Uptrend
        } else {
            -0.5 // Downtrend
        };

        let combined_signal = rsi_signal * 0.4 + sma_signal * 0.3 + sentiment * 0.3;

        if combined_signal > 0.2 {
            PredictionDirection::Buy
        } else if combined_signal < -0.2 {
            PredictionDirection::Sell
        } else {
            PredictionDirection::Hold
        }
    }

    fn calculate_confidence(&self, indicators: &TechnicalIndicators, sentiment: f64) -> f64 {
        let rsi_strength = if indicators.rsi < 30.0 || indicators.rsi > 70.0 {
            0.8
        } else {
            0.4
        };

        let trend_strength = (indicators.sma_50 - indicators.sma_200).abs() / indicators.sma_200;
        let trend_confidence = trend_strength.min(0.7);

        let sentiment_confidence = (sentiment.abs()).min(0.7);

        (rsi_strength * 0.3 + trend_confidence * 0.4 + sentiment_confidence * 0.3).min(0.99)
    }

    fn calculate_targets(
        &self,
        market_data: &MarketData,
        direction: PredictionDirection,
        atr: f64,
    ) -> (f64, f64, f64) {
        match direction {
            PredictionDirection::Buy => {
                let stop_loss = market_data.close - atr * 2.0;
                let take_profit = market_data.close + atr * 3.0;
                let target = (take_profit + market_data.close) / 2.0;
                (target, stop_loss, take_profit)
            }
            PredictionDirection::Sell => {
                let stop_loss = market_data.close + atr * 2.0;
                let take_profit = market_data.close - atr * 3.0;
                let target = (take_profit + market_data.close) / 2.0;
                (target, stop_loss, take_profit)
            }
            PredictionDirection::Hold => (market_data.close, market_data.close, market_data.close),
        }
    }
}
