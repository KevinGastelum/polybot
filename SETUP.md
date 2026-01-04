# Polymarket-Kalshi Arbitrage Bot - Setup Guide

## Quick Start for New Device

This guide will help you set up the project on a new device that doesn't have Rust installed yet.

---

## Step 1: Install Rust

### Windows (PowerShell as Administrator)
```powershell
# Download and run rustup installer
winget install Rustlang.Rustup

# Or manually:
# 1. Go to https://rustup.rs/
# 2. Download and run rustup-init.exe
# 3. Follow the on-screen instructions (choose default installation)
```

### macOS / Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, **restart your terminal** and verify:
```bash
rustc --version
cargo --version
```

---

## Step 2: Clone the Repository

```bash
git clone https://github.com/KevinGastelum/polybot.git
cd polybot
```

---

## Step 3: Create Environment File

Create a `.env` file in the project root with your credentials:

```bash
cp .env.example .env   # If .env.example exists, otherwise create manually
```

Edit `.env` with your API credentials:

```env
MCP_ENABLED=true

# Polymarket Configuration
POLYMARKET_API_KEY=your-api-key-here
POLYMARKET_SECRET=your-secret-here
POLYMARKET_PASSPHRASE=your-passphrase-here
POLYMARKET_PRIVATE_KEY=0xyour-private-key-here

POLYGON_RPC_URL=https://polygon-rpc.com

# Kalshi Configuration (API Key method - recommended)
KALSHI_API_KEY=your-kalshi-api-key
KALSHI_API_SECRET=your-kalshi-api-secret

# OR legacy email/password (deprecated by Kalshi)
KALSHI_EMAIL=your-email@example.com
KALSHI_PASSWORD=your-password

# Bot Settings
MIN_PROFIT_THRESHOLD=0.02
MAX_POSITION_SIZE=100
DRY_RUN=true
LOG_LEVEL=INFO
```

> ⚠️ **Important**: The `.env` file is in `.gitignore` and will NOT be pushed to GitHub. You need to recreate it on each device.

---

## Step 4: Build and Run

### First Build (may take a few minutes)
```bash
cargo build --release
```

### Verify API Keys
```bash
cargo run --bin verify_keys
```

Expected output:
```
✅ Polymarket Client Initialized (Credentials found)
✅ Kalshi: Logged in successfully
```

### Run the Bot (Dry Run Mode)
```bash
cargo run --bin polymarket-kalshi-arbitrage-bot
```

---

## Quick Commands Reference

| Command | Description |
|---------|-------------|
| `cargo build` | Build debug version |
| `cargo build --release` | Build optimized release version |
| `cargo run --bin verify_keys` | Verify API credentials |
| `cargo run --bin polymarket-kalshi-arbitrage-bot` | Run the arbitrage bot |
| `cargo test` | Run unit tests |

---

## Current Market Pairs (as of Jan 4, 2026)

The bot monitors these short-term Bitcoin prediction markets:

| Market | Resolution Time |
|--------|-----------------|
| BTC > $90k (Jan 4) | 9 AM EST |
| BTC > $92k (Jan 4) | 9 AM EST |
| BTC > $90k (Jan 5) | 9 AM EST |
| BTC > $92k (Jan 5) | 9 AM EST |
| BTC > $200k (Dec 31, 2026) | End of year |

> **Note**: Market IDs may need to be updated in `src/arbitrage/market_matcher.rs` for new timeframes.

---

## Troubleshooting

### "Kalshi login failed"
- Kalshi now requires API key authentication (not email/password)
- Generate API keys at https://kalshi.com/account/api
- Update `.env` with `KALSHI_API_KEY` and `KALSHI_API_SECRET`

### "Failed to parse orderbook response"
- Polymarket's CLOB API format may have changed
- Check the debug output for the actual response format
- Update `src/polymarket/types.rs` if needed

### Build errors on Windows
- Ensure Visual Studio Build Tools are installed
- Run: `rustup component add rustfmt clippy`

---

## Project Structure

```
polybot/
├── Cargo.toml           # Rust dependencies
├── .env                 # Your API credentials (create this)
├── src/
│   ├── main.rs         # Main entry point
│   ├── config.rs       # Configuration loading
│   ├── arbitrage/      # Arbitrage detection logic
│   ├── polymarket/     # Polymarket CLOB API client
│   ├── kalshi/         # Kalshi API client
│   └── utils/          # Utility functions
└── target/             # Build output (generated)
```

---

## Need Help?

1. Check the logs in the terminal output
2. Try running with `RUST_LOG=debug cargo run --bin polymarket-kalshi-arbitrage-bot`
3. Open an issue on GitHub: https://github.com/KevinGastelum/polybot/issues
