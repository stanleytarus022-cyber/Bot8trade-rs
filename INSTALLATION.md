# Installation & Configuration Guide

## Prerequisites

### System Requirements
- **OS**: Linux, macOS, or Windows (with WSL2)
- **RAM**: 4GB minimum (8GB recommended)
- **Disk**: 2GB free space
- **Internet**: Stable connection for API access

### Software Requirements
1. **Rust 1.70+**
2. **MetaTrader5** (any version)
3. **SQLite3**
4. **Git**

## Installation Steps

### Step 1: Install Rust

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (PowerShell)
irm https://win.rustup.rs -OutFile rustup-init.exe
./rustup-init.exe
```

Verify installation:
```bash
rustc --version
cargo --version
```

### Step 2: Install MetaTrader5

1. Download from [metatrader5.com](https://www.metatrader5.com)
2. Install on your system
3. Create a demo or live trading account
4. Enable WebAPI:
   - Open MT5
   - Tools → Options → Network
   - Enable "API Server" and set port (default: 5005)
   - Click OK
   - Restart MT5

### Step 3: Install SQLite3

**macOS:**
```bash
brew install sqlite3
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install sqlite3 libsqlite3-dev
```

**Windows:**
Download from [sqlite.org](https://www.sqlite.org/download.html)

### Step 4: Clone Repository

```bash
git clone https://github.com/stanleytarus022-cyber/Bot8trade-rs.git
cd Bot8trade-rs
```

### Step 5: Configure Environment

```bash
# Copy example configuration
cp .env.example .env

# Edit .env with your settings
nano .env  # or your preferred editor
```

### Step 6: Database Setup

```bash
# Initialize database
sqlite3 bot8trade.db < scripts/schema.sql

# Verify
sqlite3 bot8trade.db ".tables"
```

### Step 7: Build Project

```bash
# Build in debug mode
cargo build

# Or build release (optimized)
cargo build --release
```

## Configuration Details

### .env File

Create `.env` file with these settings:

```env
# ============================================
# MetaTrader5 Configuration
# ============================================
MT5_HOST=localhost
MT5_PORT=5005
MT5_USERNAME=your_account_number
MT5_PASSWORD=your_password

# ============================================
# API Configuration
# ============================================
# Get from https://www.alphavantage.co/
API_KEY=your_api_key

# Get from https://newsapi.org/
NEWS_API_KEY=your_news_api_key

# ============================================
# Database Configuration
# ============================================
# SQLite: sqlite://bot8trade.db
# PostgreSQL: postgresql://user:pass@localhost/bot8trade
# MySQL: mysql://user:pass@localhost/bot8trade
DATABASE_URL=sqlite://bot8trade.db

# ============================================
# Trading Settings
# ============================================
# Update interval in seconds (300 = 5 minutes)
UPDATE_INTERVAL=300

# Minimum confidence for trading (0.0 - 1.0)
MIN_CONFIDENCE=0.75

# ============================================
# Logging
# ============================================
# Levels: error, warn, info, debug, trace
RUST_LOG=info
```

## Getting API Keys

### Alpha Vantage (Market Data)
1. Go to https://www.alphavantage.co/
2. Click "Get Free API Key"
3. Enter email and accept terms
4. Copy API key to `.env`

### News API (Sentiment Analysis)
1. Go to https://newsapi.org/
2. Sign up for free account
3. Go to Account → API Keys
4. Copy API key to `.env`

### MetaTrader5 Credentials
1. Open MetaTrader5
2. Look for Account Number (usually in Account tab)
3. Use your MT5 password
4. Add to `.env`

## Testing Configuration

### Test MT5 Connection

```bash
bash scripts/test-connection.sh
```

Expected output:
```
✓ Checking prerequisites...
✓ Checking environment variables...
✓ MT5_HOST: localhost
✓ MT5_PORT: 5005
✓ Port 5005 is reachable
✓ Credentials configured
✓ API_KEY configured
✓ DATABASE_URL: sqlite://bot8trade.db
✓ All checks passed! Ready to run the bot.
```

### Test Database Connection

```bash
# Query database
sqlite3 bot8trade.db "SELECT COUNT(*) as tables FROM sqlite_master WHERE type='table';"

# Should show: tables
#             12
```

### Test API Access

```bash
# Test with curl
curl -H "Authorization: Bearer $API_KEY" \
  https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=XAUUSD&apikey=$API_KEY
```

## Running the Bot

### Basic Usage

```bash
# Run in development
cargo run

# Or release (faster)
cargo run --release
```

### With Custom Settings

```bash
# Override environment
MT5_HOST=remote.server.com cargo run --release

# Increase verbosity
RUST_LOG=debug cargo run --release
```

### Background Execution

```bash
# Use nohup
nohup cargo run --release > bot.log 2>&1 &

# Or tmux
tmux new-session -d -s bot8trade "cargo run --release"

# Monitor
tail -f bot.log
```

## Troubleshooting

### MT5 Connection Fails

**Problem**: Cannot connect to localhost:5005

**Solutions:**
1. Verify MT5 is running
2. Check Tools → Options → Network is enabled
3. Restart MT5
4. Check firewall isn't blocking port 5005
5. Try connecting from Python bridge first:
   ```bash
   python3 scripts/mt5_bridge.py
   ```

### API Key Invalid

**Problem**: 401 Unauthorized from API

**Solutions:**
1. Verify API key in `.env`
2. Check API key hasn't expired
3. Verify rate limits haven't been exceeded
4. Try API call manually:
   ```bash
   curl "https://api.example.com/endpoint?apikey=$API_KEY"
   ```

### Database Lock Error

**Problem**: "database is locked"

**Solutions:**
1. Stop the bot
2. Check for stuck processes:
   ```bash
   lsof | grep bot8trade.db
   ```
3. Delete lock files:
   ```bash
   rm -f bot8trade.db-*
   ```
4. Restart bot

### Build Failures

**Problem**: Compilation errors

**Solutions:**
1. Update Rust:
   ```bash
   rustup update
   ```
2. Clean build:
   ```bash
   cargo clean
   cargo build --release
   ```
3. Check Rust version:
   ```bash
   rustc --version  # Should be 1.70+
   ```

### Performance Issues

**Problem**: Bot running slowly

**Solutions:**
1. Use release build: `cargo build --release`
2. Increase update interval in `.env`
3. Reduce historical data retention
4. Check database indexes: `sqlite3 bot8trade.db ".indices"`

## Advanced Configuration

### Remote MT5 Server

```env
MT5_HOST=your.server.com
MT5_PORT=5005
```

### PostgreSQL Database

```bash
# Install PostgreSQL
brew install postgresql  # macOS
sudo apt-get install postgresql  # Linux

# Create database
psql -U postgres -c "CREATE DATABASE bot8trade;"

# Update .env
DATABASE_URL=postgresql://user:password@localhost/bot8trade
```

### Docker Setup

```bash
# Build image
docker build -t bot8trade .

# Run container
docker run --env-file .env -v $(pwd)/data:/data bot8trade
```

## Monitoring

### View Logs

```bash
# Real-time
tail -f logs/bot8trade.log

# Last 100 lines
tail -100 logs/bot8trade.log

# Search for errors
grep ERROR logs/bot8trade.log
```

### Check Active Trades

```bash
sqlite3 bot8trade.db "SELECT * FROM active_trades;"
```

### View Performance

```bash
sqlite3 bot8trade.db "SELECT * FROM daily_performance LIMIT 7;"
```

## Next Steps

1. ✅ Installation complete
2. ✅ Configuration done
3. 📖 Read [DEVELOPMENT.md](DEVELOPMENT.md) for code details
4. 🧪 Run tests: `cargo test`
5. 🚀 Start trading: `cargo run --release`
6. 📊 Monitor performance in database

## Getting Help

- 📚 Check [README.md](README.md)
- 🐛 Search [Issues](https://github.com/stanleytarus022-cyber/Bot8trade-rs/issues)
- 💬 Ask in [Discussions](https://github.com/stanleytarus022-cyber/Bot8trade-rs/discussions)

---

**Disclaimer**: Trading involves risk. Start with paper trading and small positions. Monitor the bot regularly.
