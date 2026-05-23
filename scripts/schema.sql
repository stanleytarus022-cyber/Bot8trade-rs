-- Bot8trade-rs Database Schema

-- Market Data Table
CREATE TABLE IF NOT EXISTS market_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL,
    symbol TEXT NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_market_data_symbol_timestamp 
    ON market_data(symbol, timestamp DESC);

-- Predictions Table
CREATE TABLE IF NOT EXISTS predictions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL,
    symbol TEXT NOT NULL,
    direction TEXT NOT NULL CHECK(direction IN ('Buy', 'Sell', 'Hold')),
    confidence REAL NOT NULL,
    target_price REAL NOT NULL,
    stop_loss REAL NOT NULL,
    take_profit REAL NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_predictions_symbol_timestamp 
    ON predictions(symbol, timestamp DESC);

-- Trades Table
CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY,
    symbol TEXT NOT NULL,
    direction TEXT NOT NULL CHECK(direction IN ('Buy', 'Sell', 'Hold')),
    entry_price REAL NOT NULL,
    quantity REAL NOT NULL,
    stop_loss REAL NOT NULL,
    take_profit REAL NOT NULL,
    opened_at DATETIME NOT NULL,
    closed_at DATETIME,
    exit_price REAL,
    status TEXT NOT NULL CHECK(status IN ('Open', 'Closed', 'Cancelled', 'Pending')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_trades_symbol_status 
    ON trades(symbol, status);
CREATE INDEX idx_trades_opened_at 
    ON trades(opened_at DESC);

-- Trade Logs Table
CREATE TABLE IF NOT EXISTS trade_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trade_id TEXT NOT NULL,
    action TEXT NOT NULL,
    details TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(trade_id) REFERENCES trades(id)
);

-- Performance Metrics Table
CREATE TABLE IF NOT EXISTS performance_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL UNIQUE,
    total_trades INTEGER DEFAULT 0,
    winning_trades INTEGER DEFAULT 0,
    losing_trades INTEGER DEFAULT 0,
    win_rate REAL DEFAULT 0.0,
    gross_profit REAL DEFAULT 0.0,
    gross_loss REAL DEFAULT 0.0,
    profit_factor REAL DEFAULT 0.0,
    max_drawdown REAL DEFAULT 0.0,
    sharpe_ratio REAL DEFAULT 0.0,
    roi REAL DEFAULT 0.0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- News Sentiment Table
CREATE TABLE IF NOT EXISTS news_sentiment (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL,
    source TEXT NOT NULL,
    sentiment_score REAL NOT NULL,
    count INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- API Calls Log (for rate limiting)
CREATE TABLE IF NOT EXISTS api_calls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    endpoint TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    status_code INTEGER,
    response_time_ms INTEGER
);

CREATE INDEX idx_api_calls_endpoint_timestamp 
    ON api_calls(endpoint, timestamp DESC);

-- System Settings
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Views for easy access

CREATE VIEW IF NOT EXISTS active_trades AS
SELECT * FROM trades
WHERE status = 'Open'
ORDER BY opened_at DESC;

CREATE VIEW IF NOT EXISTS daily_performance AS
SELECT 
    date(opened_at) as trade_date,
    COUNT(*) as total_trades,
    SUM(CASE WHEN status = 'Closed' AND exit_price > entry_price THEN 1 ELSE 0 END) as wins,
    SUM(CASE WHEN status = 'Closed' AND exit_price < entry_price THEN 1 ELSE 0 END) as losses,
    SUM(CASE WHEN status = 'Closed' THEN (exit_price - entry_price) * quantity ELSE 0 END) as daily_profit
FROM trades
GROUP BY trade_date
ORDER BY trade_date DESC;
