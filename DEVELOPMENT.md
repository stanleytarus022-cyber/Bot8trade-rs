# Bot8trade-rs Development Guide

## Project Structure

```
bot8trade-rs/
├── src/
│   ├── main.rs           # Entry point
│   ├── config.rs         # Configuration management
│   ├── api.rs            # Market data APIs
│   ├── analyzer.rs       # Technical analysis
│   ├── predictor.rs      # ML prediction engine
│   ├── mt5.rs            # MetaTrader5 integration
│   ├── trader.rs         # Main trading logic
│   ├── database.rs       # Database operations
│   ├── models.rs         # Data structures
│   └── utils.rs          # Utility functions
├── scripts/
│   ├── schema.sql        # Database schema
│   ├── mt5_bridge.py     # Python MT5 bridge
│   ├── test-connection.sh# Connection test
│   └── build.sh          # Build script
├── Cargo.toml            # Rust dependencies
├── pyproject.toml        # Python dependencies
├── .env.example          # Example configuration
└── README.md             # Project documentation
```

## Development Workflow

### 1. Setup Development Environment

```bash
# Clone and setup
git clone https://github.com/stanleytarus022-cyber/Bot8trade-rs.git
cd Bot8trade-rs

# Copy example env
cp .env.example .env

# Install dependencies
cargo build
```

### 2. Configuration

Edit `.env` file with your settings:

```env
# MetaTrader5
MT5_HOST=localhost
MT5_PORT=5005
MT5_USERNAME=your_account
MT5_PASSWORD=your_password

# API Keys
API_KEY=your_api_key
NEWS_API_KEY=your_news_key

# Database
DATABASE_URL=sqlite://bot8trade.db

# Logging
RUST_LOG=debug
```

### 3. Database Setup

```bash
# Initialize database
sqlite3 bot8trade.db < scripts/schema.sql

# Or use the build script
bash build.sh
```

### 4. Running Tests

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test test_name -- --nocapture

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Module Documentation

### config.rs
Manages environment configuration and settings.

**Key Functions:**
- `Config::from_env()` - Load configuration from environment

**Example:**
```rust
let config = Config::from_env()?;
println!("MT5 Host: {}", config.mt5_host);
```

### api.rs
Handles communication with external APIs for market data.

**Key Functions:**
- `ApiClient::new()` - Initialize API client
- `fetch_gold_price()` - Get current gold price
- `fetch_news()` - Fetch relevant news

**Example:**
```rust
let api = ApiClient::new(&config)?;
let price = api.fetch_gold_price().await?;
println!("Current price: {}", price.close);
```

### analyzer.rs
Performs technical analysis using indicators.

**Key Functions:**
- `analyze()` - Calculate all indicators
- `calculate_rsi()` - RSI indicator
- `calculate_sma()` - Simple moving average
- `calculate_atr()` - Average true range

**Example:**
```rust
let analyzer = MarketAnalyzer::new();
let indicators = analyzer.analyze(&market_data);
println!("RSI: {}", indicators.rsi);
```

### predictor.rs
Machine learning prediction engine.

**Key Functions:**
- `predict()` - Generate trade prediction
- `determine_direction()` - Buy/Sell/Hold decision
- `calculate_confidence()` - Prediction confidence score
- `calculate_targets()` - Stop loss & take profit

**Example:**
```rust
let predictor = GoldPredictor::new();
let prediction = predictor.predict(&market_data, &indicators, sentiment);
if prediction.confidence > 0.75 {
    // Execute trade
}
```

### mt5.rs
MetaTrader5 connection and order execution.

**Key Functions:**
- `MT5Connection::new()` - Initialize connection
- `execute_trade()` - Send order to MT5
- `close_trade()` - Close existing trade

**Example:**
```rust
let mt5 = MT5Connection::new(&config).await?;
let trade = mt5.execute_trade(&prediction).await?;
println!("Trade ID: {}", trade.id);
```

### trader.rs
Main trading loop and orchestration.

**Key Functions:**
- `start_trading_loop()` - Main event loop
- `run_trading_cycle()` - Single cycle execution

**Example:**
```rust
trader::start_trading_loop(api, analyzer, predictor, mt5, db).await?
```

### database.rs
Database operations and persistence.

**Key Functions:**
- `save_market_data()` - Store market data
- `save_prediction()` - Store predictions
- `save_trade()` - Store trade records
- `get_recent_market_data()` - Query historical data

**Example:**
```rust
let db = Database::new(&config).await?;
db.save_trade(&trade).await?;
```

## Adding New Features

### 1. Add a New Technical Indicator

Edit `analyzer.rs`:

```rust
pub fn calculate_macd(&self, prices: &[f64]) -> (f64, f64) {
    // Your MACD logic here
    (0.0, 0.0)
}

// Update analyze() method:
pub fn analyze(&self, market_data: &[MarketData]) -> TechnicalIndicators {
    let (macd, signal) = self.calculate_macd(&closes);
    // ...
}
```

### 2. Add a New API Source

Create `src/api/new_provider.rs`:

```rust
pub struct NewProvider {
    api_key: String,
}

impl NewProvider {
    pub async fn fetch_price(&self, symbol: &str) -> Result<f64> {
        // Implementation
    }
}
```

### 3. Add a Custom Trading Strategy

Create `src/strategies/my_strategy.rs`:

```rust
pub struct MyStrategy;

impl MyStrategy {
    pub fn analyze(&self, data: &MarketData, indicators: &TechnicalIndicators) -> Prediction {
        // Your strategy logic
    }
}
```

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Use println! Debugging

```rust
dbg!(variable);
println!("Debug: {:?}", data);
```

### Check Database

```bash
sqlite3 bot8trade.db

-- View recent trades
SELECT * FROM trades ORDER BY opened_at DESC LIMIT 5;

-- View today's performance
SELECT * FROM daily_performance LIMIT 1;
```

### Monitor Logs

```bash
tail -f logs/bot8trade.log
```

## Performance Optimization

### 1. Release Build

```bash
cargo build --release

# Run release version
./target/release/bot8trade-rs
```

### 2. Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile
cargo flamegraph --release

# View results
open flamegraph.svg
```

### 3. Benchmarking

```bash
cargo bench
```

## Deployment

### Docker Build

```bash
docker build -t bot8trade .
docker run --env-file .env bot8trade
```

### Systemd Service

```bash
sudo cp systemd/bot8trade.service /etc/systemd/system/
sudo systemctl enable bot8trade
sudo systemctl start bot8trade
sudo systemctl status bot8trade
```

## Useful Commands

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Security audit
cargo audit

# Update dependencies
cargo update

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [MetaTrader5 API](https://www.metatrader5.com/)
- [Technical Analysis](https://www.investopedia.com/)

## Support

For issues or questions:

1. Check existing [Issues](https://github.com/stanleytarus022-cyber/Bot8trade-rs/issues)
2. Create a new issue with detailed information
3. Join [Discussions](https://github.com/stanleytarus022-cyber/Bot8trade-rs/discussions)

---

**Last Updated**: 2024-05-23
