/**
 * Trade Executor - Copy Trading Bot Reference
 * Source: https://github.com/vladmeer/polymarket-copy-trading-bot
 * 
 * This is a reference implementation showing how top Polymarket copy trading bots work.
 * Key patterns:
 * 1. Trade aggregation to batch small trades
 * 2. Proportional position sizing based on balance ratios
 * 3. Position tracking to avoid duplicate trades
 */

import { ClobClient } from '@polymarket/clob-client';

// Configuration
const USER_ADDRESSES = ['0x...'];  // Traders to copy
const PROXY_WALLET = '0x...';      // Your wallet
const TRADE_AGGREGATION_WINDOW_SECONDS = 30;
const TRADE_AGGREGATION_MIN_TOTAL_USD = 1.0;

interface TradeWithUser {
    userAddress: string;
    conditionId: string;
    asset: string;
    side: string;
    usdcSize: number;
    price: number;
    slug?: string;
    eventSlug?: string;
    transactionHash: string;
}

interface AggregatedTrade {
    userAddress: string;
    conditionId: string;
    asset: string;
    side: string;
    trades: TradeWithUser[];
    totalUsdcSize: number;
    averagePrice: number;
    firstTradeTime: number;
    lastTradeTime: number;
}

// Buffer for aggregating trades
const tradeAggregationBuffer: Map<string, AggregatedTrade> = new Map();

/**
 * Generate a unique key for trade aggregation based on user, market, side
 */
const getAggregationKey = (trade: TradeWithUser): string => {
    return `${trade.userAddress}:${trade.conditionId}:${trade.asset}:${trade.side}`;
};

/**
 * Add trade to aggregation buffer or update existing aggregation
 */
const addToAggregationBuffer = (trade: TradeWithUser): void => {
    const key = getAggregationKey(trade);
    const existing = tradeAggregationBuffer.get(key);
    const now = Date.now();

    if (existing) {
        // Update existing aggregation
        existing.trades.push(trade);
        existing.totalUsdcSize += trade.usdcSize;
        // Recalculate weighted average price
        const totalValue = existing.trades.reduce((sum, t) => sum + t.usdcSize * t.price, 0);
        existing.averagePrice = totalValue / existing.totalUsdcSize;
        existing.lastTradeTime = now;
    } else {
        // Create new aggregation
        tradeAggregationBuffer.set(key, {
            userAddress: trade.userAddress,
            conditionId: trade.conditionId,
            asset: trade.asset,
            side: trade.side || 'BUY',
            trades: [trade],
            totalUsdcSize: trade.usdcSize,
            averagePrice: trade.price,
            firstTradeTime: now,
            lastTradeTime: now,
        });
    }
};

/**
 * Check buffer and return ready aggregated trades
 * Trades are ready if:
 * 1. Total size >= minimum AND
 * 2. Time window has passed since first trade
 */
const getReadyAggregatedTrades = (): AggregatedTrade[] => {
    const ready: AggregatedTrade[] = [];
    const now = Date.now();
    const windowMs = TRADE_AGGREGATION_WINDOW_SECONDS * 1000;

    for (const [key, agg] of tradeAggregationBuffer.entries()) {
        const timeElapsed = now - agg.firstTradeTime;

        if (timeElapsed >= windowMs) {
            if (agg.totalUsdcSize >= TRADE_AGGREGATION_MIN_TOTAL_USD) {
                ready.push(agg);
            }
            tradeAggregationBuffer.delete(key);
        }
    }

    return ready;
};

/**
 * Execute aggregated trades with proportional sizing
 */
const doAggregatedTrading = async (clobClient: ClobClient, aggregatedTrades: AggregatedTrade[]) => {
    for (const agg of aggregatedTrades) {
        console.log(`📊 AGGREGATED TRADE (${agg.trades.length} trades combined)`);
        console.log(`Market: ${agg.asset}`);
        console.log(`Side: ${agg.side}`);
        console.log(`Total volume: $${agg.totalUsdcSize.toFixed(2)}`);
        console.log(`Average price: $${agg.averagePrice.toFixed(4)}`);

        // Fetch positions for sizing calculation
        const my_positions = await fetch(
            `https://data-api.polymarket.com/positions?user=${PROXY_WALLET}`
        ).then(r => r.json());
        
        const user_positions = await fetch(
            `https://data-api.polymarket.com/positions?user=${agg.userAddress}`
        ).then(r => r.json());

        // Get my USDC balance
        const my_balance = await getMyBalance(PROXY_WALLET);

        // Calculate trader's total portfolio value from positions
        const user_balance = user_positions.reduce((total: number, pos: any) => {
            return total + (pos.currentValue || 0);
        }, 0);

        // Calculate proportional position size
        const balanceRatio = my_balance / user_balance;
        const myTradeSize = agg.totalUsdcSize * balanceRatio;

        console.log(`My balance: $${my_balance.toFixed(2)}`);
        console.log(`Trader balance: $${user_balance.toFixed(2)}`);
        console.log(`Balance ratio: ${(balanceRatio * 100).toFixed(2)}%`);
        console.log(`My trade size: $${myTradeSize.toFixed(2)}`);

        // Execute the trade using CLOB client
        // await clobClient.createOrder({ ... });
    }
};

async function getMyBalance(wallet: string): Promise<number> {
    // Implementation would fetch USDC balance from blockchain
    return 1000; // Placeholder
}

export { addToAggregationBuffer, getReadyAggregatedTrades, doAggregatedTrading };
