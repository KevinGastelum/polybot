/**
 * Trade Monitor - Copy Trading Bot Reference
 * Source: https://github.com/vladmeer/polymarket-copy-trading-bot
 * 
 * This is a reference implementation showing how to monitor trader activity.
 * Key patterns:
 * 1. Poll activity API at regular intervals
 * 2. Track processed trades to avoid duplicates
 * 3. Update position snapshots for portfolio analysis
 */

// Configuration
const USER_ADDRESSES = ['0x...'];  // Traders to monitor
const TOO_OLD_TIMESTAMP = Date.now() - (24 * 60 * 60 * 1000);  // 24 hours ago
const FETCH_INTERVAL = 5000;  // 5 seconds

interface UserActivity {
    proxyWallet: string;
    timestamp: number;
    conditionId: string;
    type: string;  // 'TRADE'
    size: number;
    usdcSize: number;
    transactionHash: string;
    price: number;
    asset: string;
    side: string;  // 'BUY' or 'SELL'
    outcomeIndex: number;
    title: string;
    slug: string;
    eventSlug: string;
    outcome: string;
}

interface UserPosition {
    proxyWallet: string;
    asset: string;
    conditionId: string;
    size: number;
    avgPrice: number;
    initialValue: number;
    currentValue: number;
    cashPnl: number;
    percentPnl: number;
    totalBought: number;
    realizedPnl: number;
    curPrice: number;
    title: string;
    slug: string;
    eventSlug: string;
    outcome: string;
    outcomeIndex: number;
}

// In-memory store for processed trades (use MongoDB in production)
const processedTrades = new Set<string>();
const positions = new Map<string, UserPosition[]>();

/**
 * Initialize monitoring - fetch current state
 */
const init = async () => {
    console.log('Initializing trade monitor...');
    
    for (const address of USER_ADDRESSES) {
        // Fetch current positions
        const positionsUrl = `https://data-api.polymarket.com/positions?user=${address}`;
        const response = await fetch(positionsUrl);
        const userPositions = await response.json();
        
        if (Array.isArray(userPositions)) {
            positions.set(address, userPositions);
            
            // Calculate overall PnL
            let totalValue = 0;
            let weightedPnl = 0;
            userPositions.forEach((pos: UserPosition) => {
                totalValue += pos.currentValue || 0;
                weightedPnl += (pos.currentValue || 0) * (pos.percentPnl || 0);
            });
            const overallPnl = totalValue > 0 ? weightedPnl / totalValue : 0;
            
            console.log(`Trader ${address.slice(0, 8)}...:`);
            console.log(`  Positions: ${userPositions.length}`);
            console.log(`  Total Value: $${totalValue.toFixed(2)}`);
            console.log(`  Overall PnL: ${(overallPnl * 100).toFixed(2)}%`);
        }
    }
};

/**
 * Fetch new trade activities from Polymarket API
 */
const fetchTradeData = async (): Promise<UserActivity[]> => {
    const newTrades: UserActivity[] = [];
    
    for (const address of USER_ADDRESSES) {
        try {
            // Fetch trade activities from Polymarket API
            const apiUrl = `https://data-api.polymarket.com/activity?user=${address}&type=TRADE`;
            const response = await fetch(apiUrl);
            const activities = await response.json();

            if (!Array.isArray(activities) || activities.length === 0) {
                continue;
            }

            // Process each activity
            for (const activity of activities) {
                // Skip if too old
                if (activity.timestamp < TOO_OLD_TIMESTAMP) {
                    continue;
                }

                // Check if this trade already processed
                if (processedTrades.has(activity.transactionHash)) {
                    continue;
                }

                // Mark as processed
                processedTrades.add(activity.transactionHash);
                
                console.log(`New trade detected for ${address.slice(0, 8)}...`);
                console.log(`  ${activity.side} ${activity.outcome} @ $${activity.price}`);
                console.log(`  Size: $${activity.usdcSize.toFixed(2)}`);
                console.log(`  Market: ${activity.title}`);
                
                newTrades.push(activity);
            }

            // Also update positions snapshot
            const positionsUrl = `https://data-api.polymarket.com/positions?user=${address}`;
            const positionsResponse = await fetch(positionsUrl);
            const userPositions = await positionsResponse.json();
            
            if (Array.isArray(userPositions)) {
                positions.set(address, userPositions);
            }
        } catch (error) {
            console.error(`Error fetching data for ${address}: ${error}`);
        }
    }
    
    return newTrades;
};

/**
 * Start the monitoring loop
 */
const startMonitoring = async (onNewTrades: (trades: UserActivity[]) => void) => {
    await init();
    
    console.log('Starting trade monitoring loop...');
    
    setInterval(async () => {
        const newTrades = await fetchTradeData();
        if (newTrades.length > 0) {
            onNewTrades(newTrades);
        }
    }, FETCH_INTERVAL);
};

export { init, fetchTradeData, startMonitoring, UserActivity, UserPosition };
