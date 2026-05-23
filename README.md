# Bot8trade-rs: Advanced Gold Trading Bot

A high-performance, production-ready trading bot written in Rust for gold market prediction and automated trading with MetaTrader5 integration.

## Features

✅ **Market Analysis**
- Real-time gold price tracking
- Technical indicators (RSI, MACD, Bollinger Bands, ATR, SMA)
- Advanced pattern recognition
- Multi-timeframe analysis

✅ **AI/ML Prediction**
- Machine learning models for price prediction
- Sentiment analysis from financial news
- Market trend detection
- Confidence scoring

✅ **Trading Automation**
- Automated order execution
- Risk management (stop-loss, take-profit)
- Position sizing algorithms
- Trade logging and analytics

✅ **MetaTrader5 Integration**
- Direct MT5 connection
- Live order execution
- Real-time account updates
- WebAPI support

✅ **Data Management**
- SQLite database for historical data
- Trade history tracking
- Performance analytics
- Backtesting capabilities

## Quick Start

### Prerequisites
- Rust 1.70+
- MetaTrader5 installed
- SQLite3
- API keys for market data providers

### Installation

```bash
git clone https://github.com/stanleytarus022-cyber/Bot8trade-rs.git
cd Bot8trade-rs
cargo build --release
```

### Configuration

Create a `.env` file:

```env
# MetaTrader5
MT5_HOST=localhost
MT5_PORT=5005
MT5_USERNAME=your_mt5_username
MT5_PASSWORD=your_mt5_password

# API Keys
API_KEY=your_market_data_api_key
NEWS_API_KEY=your_news_api_key

# Database
DATABASE_URL=sqlite://bot8trade.db

# Settings
UPDATE_INTERVAL=300  # seconds
MIN_CONFIDENCE=0.75   # 75%
```

### Running the Bot

```bash
cargo run --release
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│           Bot8trade-rs Core                      │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────┐  ┌──────────────┐            │
│  │  API Client  │  │   Analyzer   │            │
│  │              │  │              │            │
│  │ • Market Data│  │ • RSI        │            │
│  │ • News/News │  │ • MACD       │            │
│  │ • Sentiment │  │ • Bollinger  │            │
│  └──────────────┘  └──────────────┘            │
│         │                 │                     │
│         └─────────┬───────┘                     │
│                   │                             │
│           ┌───────▼────────┐                   │
│           │   Predictor    │                   │
│           │                │                   │
│           │ • ML Model     │                   │
│           │ • Direction    │                   │
│           │ • Confidence   │                   │
│           └────────┬────────┘                   │
│                    │                            │
│           ┌────────▼────────┐                  │
│           │      Trader     │                  │
│           │                 │                  │
│           │ • Risk Mgmt     │                  │
│           │ • Order Exec    │                  │
│           └────────┬────────┘                  │
│                    │                            │
│           ┌────────▼────────┐                  │
│           │   MT5 Bridge    │                  │
│           │                 │                  │
│           │ • Live Trading  │                  │
│           │ • Account Info  │                  │
│           └─────────────────┘                  │
│                    │                            │
│           ┌────────▼────────┐                  │
│           │   Database      │                  │
│           │   (SQLite)      │                  │
│           └─────────────────┘                  │
│                                                  │
└─────────────────────────────────────────────────┘
```

## MetaTrader5 Integration Guide

### Step 1: Enable MT5 WebAPI

1. Open MetaTrader5
2. Go to **Tools** → **Options** → **Network**
3. Enable WebAPI and set port (default: 5005)
4. Save and restart MT5

### Step 2: Configure Bot Connection

Update `.env` with your MT5 credentials:
```env
MT5_HOST=127.0.0.1
MT5_PORT=5005
MT5_USERNAME=your_account_number
MT5_PASSWORD=your_password
```

### Step 3: Run the Bot

```bash
cargo run --release
```

The bot will automatically:
- Connect to MT5
- Fetch market data
- Analyze price movements
- Execute trades based on predictions
- Log all activity

### Step 4: Monitor Performance

View real-time logs:
```bash
tail -f logs/bot8trade.log
```

Database queries:
```bash
sqlite3 bot8trade.db
> SELECT * FROM trades ORDER BY opened_at DESC LIMIT 10;
```

## API Integration

### Supported Data Providers

- **Alpha Vantage** - Real-time gold prices
- **IEX Cloud** - Market data and news
- **Finnhub** - Financial news and sentiment
- **MetaQuotes** - Direct MT5 data

### Adding a New API

1. Create a new module in `src/api/`
2. Implement the `DataProvider` trait
3. Register in `src/api/mod.rs`

## Advanced Features

### Backtesting

```bash
cargo run --release --features backtesting -- --backtest --from 2024-01-01 --to 2024-12-31
```

### Paper Trading (Risk-Free)

```bash
cargo run --release -- --paper-trading
```

### Custom Strategy

Create `src/strategies/my_strategy.rs`:

```rust
use crate::models::*;

pub struct MyStrategy;

impl Strategy for MyStrategy {
    fn analyze(&self, data: &MarketData) -> Prediction {
        // Your custom logic here
    }
}
```

## Performance Metrics

The bot tracks:
- **Win Rate**: % of profitable trades
- **Profit Factor**: Gross profit / Gross loss
- **Sharpe Ratio**: Risk-adjusted returns
- **Drawdown**: Maximum loss from peak
- **ROI**: Return on investment

View statistics:
```bash
sqlite3 bot8trade.db "SELECT * FROM performance_metrics;"
```

## Risk Management

⚠️ **Important Safety Features**

1. **Position Sizing**: Automatically calculates lot size based on risk tolerance
2. **Stop Loss**: Required on all trades
3. **Take Profit**: Locks in gains
4. **Max Drawdown Limit**: Stops trading if losses exceed threshold
5. **Daily Loss Limit**: Prevents excessive daily losses

## Troubleshooting

### MT5 Connection Issues

```bash
# Check MT5 is running
lsof -i :5005

# Verify credentials
cargo run --release -- --test-connection

# Check firewall
sudo ufw allow 5005
```

### Market Data Issues

```bash
# Test API connection
curl -H "Authorization: Bearer $API_KEY" https://api.example.com/gold/price

# Check API quota
sqlite3 bot8trade.db "SELECT COUNT(*) FROM api_calls WHERE date > date('now', '-1 day');"
```

### Performance Issues

```bash
# Enable debug logging
RUST_LOG=debug cargo run --release

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --release
```

## Development

### Running Tests

```bash
cargo test --all
cargo test --release
```

### Code Quality

```bash
cargo fmt              # Format code
cargo clippy          # Lint
cargo audit           # Security check
```

### Building Documentation

```bash
cargo doc --open
```

## Deployment

### Docker

```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/bot8trade-rs"]
```

### Systemd Service

```ini
[Unit]
Description=Bot8trade-rs Trading Bot
After=network.target

[Service]
Type=simple
User=bot8trade
WorkingDirectory=/opt/bot8trade-rs
ExecStart=/opt/bot8trade-rs/target/release/bot8trade-rs
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Submit a pull request
4. Ensure all tests pass

## License

GNU General Public License v3.0

## Disclaimer

⚠️ **TRADING RISK**: Automated trading involves substantial risk. Past performance does not guarantee future results. Use paper trading first. Always validate predictions manually. This bot is provided as-is without warranty.

## Support

- 📖 [Documentation](https://github.com/stanleytarus022-cyber/Bot8trade-rs/wiki)
- 🐛 [Issue Tracker](https://github.com/stanleytarus022-cyber/Bot8trade-rs/issues)
- 💬 [Discussions](https://github.com/stanleytarus022-cyber/Bot8trade-rs/discussions)

## Author

stanleytarus022-cyber

---

**Last Updated**: 2024-05-23
**Version**: 0.1.0
