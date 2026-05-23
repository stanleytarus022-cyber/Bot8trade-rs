# MetaTrader5 Integration Complete Guide

## 🎯 Overview

This guide provides everything you need to integrate Bot8trade-rs with MetaTrader5 and deploy a production-ready automated trading bot.

---

## 📋 Table of Contents

1. [MetaTrader5 Setup](#metatrader5-setup)
2. [Bot Configuration](#bot-configuration)
3. [Connection & Testing](#connection--testing)
4. [Running the Bot](#running-the-bot)
5. [Monitoring & Maintenance](#monitoring--maintenance)
6. [Advanced Features](#advanced-features)
7. [Troubleshooting](#troubleshooting)

---

## MetaTrader5 Setup

### Step 1: Enable WebAPI in MT5

#### For MetaTrader5 (Forex/Stocks)

1. **Open MetaTrader5**
2. Go to **Tools** → **Options**
3. Navigate to **Network** tab
4. Check "API Server" box
5. Set port (default: **5005**)
6. Click **OK**
7. **Restart MetaTrader5** to apply changes

#### Verify WebAPI is Running

```bash
# Test connection
netstat -an | grep 5005

# Should show LISTENING on port 5005
# Windows: netstat -ano | findstr :5005
```

### Step 2: Get Your Account Credentials

1. Open MetaTrader5
2. Look at the **Account** tab (usually top-left corner)
3. Note down:
   - **Account Number** (your username for the bot)
   - **Server Name** (e.g., "Roboforex-Demo")
   - Your **Password**

### Step 3: Create or Use Trading Account

Choices:
- **Demo Account** (Recommended for testing)
  - Risk-free practice
  - Real market data
  - No real money
  - Perfect for testing bot strategies

- **Live Account** (Production)
  - Real money trading
  - Requires proper testing first
  - Enable 2FA for security

---

## Bot Configuration

### Step 1: Create Configuration File

```bash
cd Bot8trade-rs
cp .env.example .env
```

### Step 2: Edit .env File

```env
# ============================================
# MetaTrader5 Connection
# ============================================
MT5_HOST=localhost          # Your MT5 machine (localhost if same computer)
MT5_PORT=5005              # WebAPI port (default)
MT5_USERNAME=12345678      # Your account number
MT5_PASSWORD=your_password  # Your MT5 password

# ============================================
# Market Data APIs (Choose at least one)
# ============================================
# Alpha Vantage (recommended for gold)
API_KEY=your_alphavantage_api_key

# NewsAPI for sentiment analysis
NEWS_API_KEY=your_newsapi_key

# ============================================
# Database
# ============================================
DATABASE_URL=sqlite://bot8trade.db

# ============================================
# Trading Parameters
# ============================================
# Update cycle (300 = 5 minutes)
UPDATE_INTERVAL=300

# Only trade when confidence > this threshold
MIN_CONFIDENCE=0.75

# ============================================
# Logging
# ============================================
RUST_LOG=info              # Levels: error, warn, info, debug, trace
```

### Step 3: Get API Keys

#### Alpha Vantage (Market Data)

1. Visit: https://www.alphavantage.co/
2. Click "Get Free API Key"
3. Enter email and accept terms
4. Copy API key to `.env`

Free tier: 5 calls/minute, 500 calls/day

#### NewsAPI (Sentiment Analysis)

1. Visit: https://newsapi.org/
2. Sign up for free
3. Go to "Account" → "API Keys"
4. Copy API key to `.env`

Free tier: 100 requests/day

---

## Connection & Testing

### Test 1: MT5 Connection

```bash
# Make test script executable
chmod +x scripts/test-connection.sh

# Run connection test
bash scripts/test-connection.sh
```

### Test 2: Python Bridge Test

```bash
# Test MT5 connection via Python
python3 scripts/mt5_bridge.py
```

### Test 3: API Connectivity

```bash
# Test market data API
curl -s "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=XAUUSD&apikey=$API_KEY" | python -m json.tool
```

### Test 4: Database Setup

```bash
# Initialize database
sqlite3 bot8trade.db < scripts/schema.sql

# Verify tables created
sqlite3 bot8trade.db ".tables"
```

---

## Running the Bot

### Before First Run

**Checklist:**
- [ ] MetaTrader5 running with WebAPI enabled
- [ ] All `.env` variables configured
- [ ] API keys obtained and verified
- [ ] Database initialized
- [ ] Test script passed
- [ ] Using DEMO account (first time)

### Start the Bot

#### Option 1: Direct Execution

```bash
# Debug mode (verbose logging)
RUST_LOG=debug cargo run

# Or release mode (optimized, faster)
cargo run --release
```

#### Option 2: Background Execution

```bash
# Using nohup
nohup cargo run --release > bot8trade.log 2>&1 &

# Using tmux
tmux new-session -d -s bot8trade "cargo run --release"

# Monitor logs
tail -f bot8trade.log
```

#### Option 3: Docker Execution

```bash
# Build Docker image
docker build -t bot8trade-rs .

# Run container
docker run --env-file .env \
  --volume $(pwd)/data:/data \
  -d \
  bot8trade-rs
```

---

## Monitoring & Maintenance

### Real-Time Monitoring

#### View Active Trades

```bash
sqlite3 bot8trade.db "SELECT * FROM active_trades;"
```

#### Check Daily Performance

```bash
sqlite3 bot8trade.db "SELECT * FROM daily_performance ORDER BY trade_date DESC LIMIT 7;"
```

#### View Performance Metrics

```bash
sqlite3 bot8trade.db "SELECT * FROM performance_metrics ORDER BY date DESC LIMIT 1;"
```

### Log Analysis

```bash
# View recent logs
tail -50 logs/bot8trade.log

# Search for errors
grep ERROR logs/bot8trade.log | tail -10

# Find failed API calls
grep "API error" logs/bot8trade.log
```

---

## Advanced Features

### 1. Paper Trading (Risk-Free)

```bash
# Start in paper trading mode
cargo run --release -- --paper-trading
```

### 2. Backtesting

```bash
# Test strategy on historical data
cargo run --release --features backtesting -- \
  --backtest \
  --from 2024-01-01 \
  --to 2024-05-23 \
  --initial-capital 10000
```

### 3. Multiple Symbols

Edit `src/trader.rs` to add more symbols like SILVER, COPPER, etc.

### 4. Webhook Integration

Set up alerts via webhook for important trade events.

---

## Troubleshooting

### Connection Issues

**Problem:** "Cannot connect to localhost:5005"

**Solutions:**
1. Verify MT5 is running
2. Check WebAPI is enabled: Tools → Options → Network
3. Verify port 5005 is open: `netstat -an | grep 5005`
4. Restart MT5

### Authentication Issues

**Problem:** "Authentication error" from MT5

**Solutions:**
1. Verify account number is correct
2. Check password is correct
3. Ensure account is active (not archived)
4. Try logging in to MT5 manually

### API Key Issues

**Problem:** "401 Unauthorized" from API

**Solutions:**
1. Verify API key in `.env`
2. Check API key is correct (no extra spaces)
3. Verify API key hasn't expired
4. Check you haven't exceeded rate limits

### Database Issues

**Problem:** "database is locked"

**Solutions:**
```bash
# Stop the bot
pkill -f bot8trade

# Remove lock files
rm -f bot8trade.db-*

# Restart bot
cargo run --release
```

---

## Best Practices

### Security

✅ Do:
- Use strong MT5 password
- Store `.env` securely (add to `.gitignore`)
- Enable 2FA on trading account
- Use demo account for testing
- Audit trade logs regularly

❌ Don't:
- Share API keys
- Commit `.env` to git
- Use weak passwords
- Trade with more than you can afford to lose

### Risk Management

✅ Do:
- Start with small position sizes
- Always set stop loss
- Define take profit levels
- Monitor P&L daily
- Test strategy on demo first

❌ Don't:
- Over-leverage
- Skip stop loss
- Trade with scared money
- Hold positions overnight (initially)
- Risk more than 2% per trade

---

## Performance Targets

| Metric | Target | Warning |
|--------|--------|---------|
| API Response Time | < 1s | > 2s |
| Analysis Time | < 100ms | > 500ms |
| Win Rate | > 50% | < 40% |
| Max Drawdown | < 10% | > 20% |
| Memory Usage | < 500MB | > 1GB |

---

## Deployment Checklist

### Pre-Deployment
- [ ] All tests passing: `cargo test`
- [ ] Code formatted: `cargo fmt`
- [ ] No warnings: `cargo clippy`
- [ ] Configuration complete and verified
- [ ] Database initialized and tested
- [ ] API keys obtained and tested
- [ ] MT5 connection tested

### Deployment
- [ ] Use release build: `cargo build --release`
- [ ] Start in demo mode first
- [ ] Monitor for 24+ hours on demo
- [ ] Analyze trades and performance

### Post-Deployment
- [ ] Monitor daily for first week
- [ ] Review logs regularly
- [ ] Check database integrity
- [ ] Track P&L

---

## ⚠️ Disclaimer

**Trading involves substantial risk of loss. Past performance does not guarantee future results.**

This bot is provided for educational purposes. Use at your own risk. Test thoroughly on demo accounts before using real money.

---

**Last Updated:** 2024-05-23  
**Version:** 1.0.0
