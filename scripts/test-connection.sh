#!/bin/bash

# MetaTrader5 Connection Test

echo "🔍 Testing MetaTrader5 Connection"
echo "==================================="
echo ""

# Check environment variables
echo "1️⃣  Checking environment variables..."
if [ -z "$MT5_HOST" ] || [ -z "$MT5_PORT" ]; then
    echo "❌ MT5_HOST or MT5_PORT not set"
    echo "   Set them in .env file"
    exit 1
fi
echo "✅ MT5_HOST: $MT5_HOST"
echo "✅ MT5_PORT: $MT5_PORT"

# Test network connectivity
echo ""
echo "2️⃣  Testing network connectivity..."
if timeout 3 bash -c "cat < /dev/null > /dev/tcp/$MT5_HOST/$MT5_PORT" 2>/dev/null; then
    echo "✅ Port $MT5_PORT is reachable"
else
    echo "❌ Cannot connect to $MT5_HOST:$MT5_PORT"
    echo "   Make sure MetaTrader5 is running with WebAPI enabled"
    echo "   Check: Tools → Options → Network"
    exit 1
fi

# Test credentials
echo ""
echo "3️⃣  Testing credentials..."
if [ -z "$MT5_USERNAME" ] || [ -z "$MT5_PASSWORD" ]; then
    echo "⚠️  MT5_USERNAME or MT5_PASSWORD not set"
    echo "   These will be needed for trading"
else
    echo "✅ Credentials configured"
fi

# Test API key
echo ""
echo "4️⃣  Testing API key..."
if [ -z "$API_KEY" ]; then
    echo "⚠️  API_KEY not set"
    echo "   Market data fetching will not work"
else
    echo "✅ API_KEY configured"
fi

echo ""
echo "5️⃣  Database configuration..."
DB_URL=${DATABASE_URL:-sqlite://bot8trade.db}
echo "✅ DATABASE_URL: $DB_URL"

echo ""
echo "✅ All checks passed! Ready to run the bot."
echo ""
echo "Run: cargo run --release"
