# Agent Mission

**Objective:** Build a high-performance Polymarket-Kalshi arbitrage trading bot.

## Description
This bot monitors price discrepancies between Polymarket (Polygon-based prediction market) and Kalshi (US-regulated prediction market) to identify and execute risk-free arbitrage opportunities in real-time.

### Core Capabilities
1. **Real-time Market Monitoring**: WebSocket connections to both platforms for sub-millisecond price updates.
2. **Arbitrage Detection**: SIMD-accelerated algorithms to identify when YES + NO prices sum to less than $1.00.
3. **Automated Execution**: Place simultaneous orders on both platforms to lock in guaranteed profits.
4. **Position Tracking**: Monitor open positions and P&L in real-time.

## Technology Stack
- **Language**: Rust (for maximum performance and safety)
- **Async Runtime**: Tokio
- **Ethereum Interaction**: ethers-rs
- **API Communication**: reqwest + tokio-tungstenite (WebSockets)

## Success Criteria
- Bot successfully connects to both Polymarket and Kalshi APIs.
- Arbitrage opportunities are detected with latency < 10ms.
- Trades are executed atomically on both platforms.
- Comprehensive logging and error handling.
- Dry-run mode for testing without real funds.

## Risk Considerations
> **IMPORTANT**: This bot executes real trades. Ensure:
> - API keys are stored securely in `.env` (never committed).
> - Strict position limits are configured.
> - Circuit breakers are enabled for anomaly detection.
