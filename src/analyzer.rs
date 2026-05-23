use crate::models::{MarketData, TechnicalIndicators};
use log::debug;

pub struct MarketAnalyzer {
    // Historical data buffer
}

impl MarketAnalyzer {
    pub fn new() -> Self {
        MarketAnalyzer {}
    }

    pub fn calculate_rsi(&self, prices: &[f64], period: usize) -> f64 {
        if prices.len() < period + 1 {
            return 50.0;
        }

        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..=period {
            let change = prices[prices.len() - i] - prices[prices.len() - i - 1];
            if change > 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }

        let avg_gain = gains / period as f64;
        let avg_loss = losses / period as f64;

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }

    pub fn calculate_sma(&self, prices: &[f64], period: usize) -> f64 {
        if prices.len() < period {
            return prices.iter().sum::<f64>() / prices.len() as f64;
        }
        prices[prices.len() - period..].iter().sum::<f64>() / period as f64
    }

    pub fn calculate_atr(&self, highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> f64 {
        if highs.len() < period {
            return 0.0;
        }

        let mut tr_sum = 0.0;
        for i in (closes.len() - period)..closes.len() {
            let tr = (highs[i] - lows[i])
                .max((highs[i] - closes[i - 1]).abs())
                .max((lows[i] - closes[i - 1]).abs());
            tr_sum += tr;
        }

        tr_sum / period as f64
    }

    pub fn analyze(&self, market_data: &[MarketData]) -> TechnicalIndicators {
        if market_data.is_empty() {
            return TechnicalIndicators {
                rsi: 50.0,
                macd: 0.0,
                bollinger_upper: 0.0,
                bollinger_lower: 0.0,
                sma_50: 0.0,
                sma_200: 0.0,
                atr: 0.0,
            };
        }

        let closes: Vec<f64> = market_data.iter().map(|m| m.close).collect();
        let highs: Vec<f64> = market_data.iter().map(|m| m.high).collect();
        let lows: Vec<f64> = market_data.iter().map(|m| m.low).collect();

        let rsi = self.calculate_rsi(&closes, 14);
        let sma_50 = self.calculate_sma(&closes, 50);
        let sma_200 = self.calculate_sma(&closes, 200);
        let atr = self.calculate_atr(&highs, &lows, &closes, 14);

        debug!("RSI: {}, SMA50: {}, SMA200: {}, ATR: {}", rsi, sma_50, sma_200, atr);

        TechnicalIndicators {
            rsi,
            macd: 0.0, // TODO: Implement MACD
            bollinger_upper: 0.0, // TODO: Implement Bollinger Bands
            bollinger_lower: 0.0,
            sma_50,
            sma_200,
            atr,
        }
    }
}
