# Polymarket Trading Bot Reference Code

This directory contains reference code samples from top Polymarket trading bots.
Use these as learning resources to understand successful trading strategies.

## Sources

### 1. Official Polymarket Clients
- **py-clob-client**: https://github.com/Polymarket/py-clob-client
- **agents**: https://github.com/Polymarket/agents (AI trading framework)

### 2. Community Trading Bots
- **vladmeer/polymarket-copy-trading-bot**: https://github.com/vladmeer/polymarket-copy-trading-bot
  - Copy trading bot with trade aggregation
  - TypeScript/Node.js implementation

## Key APIs Discovered

### Polymarket Data APIs
```
# Leaderboard
https://data-api.polymarket.com/v1/leaderboard?timePeriod=monthly&orderBy=PNL&limit=20&category=overall

# User Positions
https://data-api.polymarket.com/positions?user=<address>&limit=50

# User Activity/Trades
https://data-api.polymarket.com/activity?user=<address>&limit=25

# User PnL History
https://user-pnl-api.polymarket.com/user-pnl?user_address=<address>&interval=1m&fidelity=1d

# Markets Traded by User
https://data-api.polymarket.com/traded?user=<address>
```

## Top Traders to Monitor (January 2026)

| Rank | Name | Address | Monthly Profit |
|------|------|---------|----------------|
| 1 | SeriouslySirius | `0x16b29c50f2439faf627209b2ac0c7bbddaa8a881` | +$1.5M |
| 2 | DrPufferfish | `0xdb27bf2ac5d428a9c63dbc914611036855a6c56e` | +$772K |
| 3 | kch123 | `0x6a72f61820b26b1fe4d956e17b6dc2a1ea3033ee` | +$693K |
| 4 | SemyonMarmeladov | `0x37e4728b3c4607fb2b3b205386bb1d1fb1a8c991` | +$589K |
| 5 | swisstony | `0x204f72f35326db932158cba6adff0b9a1da95e14` | +$518K |

### All-Time Leaders
- **Theo4**: `0x56687bf447db6ffa42ffe2204a05edaa20f55839` (+$22M)
- **Fredi9999**: `0x1f2dd6d473f3e824cd2f8a89d9c69fb96f6ad0cf` (+$16M)

## Key Trading Patterns Observed

### 1. Trade Aggregation
Top bots aggregate small trades into larger positions to minimize fees:
```typescript
// Buffer trades by: user + market + side
const key = `${trade.userAddress}:${trade.conditionId}:${trade.asset}:${trade.side}`;
// Execute when window passes OR total size exceeds minimum
```

### 2. Proportional Position Sizing
Scale positions based on your balance vs. trader's balance:
```typescript
const my_balance = await getMyBalance(PROXY_WALLET);
const user_balance = user_positions.reduce((total, pos) => total + pos.currentValue, 0);
// Position size = trade.usdcSize * (my_balance / user_balance)
```

### 3. Real-Time Activity Monitoring
Poll the activity API to detect new trades:
```typescript
const apiUrl = `https://data-api.polymarket.com/activity?user=${address}&type=TRADE`;
const activities = await fetchData(apiUrl);
// Filter by timestamp to get only new trades
```

## Files in This Directory

- `README.md` - This file
- `trade_executor_sample.ts` - Copy trading execution logic
- `trade_monitor_sample.ts` - Activity monitoring logic
- `polymarket_client_sample.py` - Official Python client reference
