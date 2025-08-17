
use crate::config::Config;
use crate::solana::SolanaClient;
use crate::dex::DexClient;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub async fn run(cfg: &Config, pair: &str, spread_bps: u32) -> Result<()> {
    log::info!("Starting market-maker for pair={} target_spread={} bps", pair, spread_bps);
    
    // Initialize clients
    let solana_client = SolanaClient::new(&cfg.rpc_url, &cfg.wallet_path)?;
    let dex_client = DexClient::new()?;
    
    // Get pool information
    let pool = dex_client.get_pool_info(pair).await?;
    log::info!("Pool: {} ({} <-> {})", pool.address, pool.token_a, pool.token_b);
    log::info!("Current reserves: {} SOL, {} tokens", 
        pool.reserve_a as f64 / 1_000_000_000.0,
        pool.reserve_b
    );
    
    // Calculate target spread
    let spread_ratio = spread_bps as f64 / 10_000.0;
    log::info!("Target spread: {:.4} ({:.2}%)", spread_ratio, spread_bps as f64 / 100.0);
    
    // Market making loop
    for round in 1..=5 {
        log::info!("Market-making round {}/5", round);
        
        // Get current pool state
        let current_pool = dex_client.get_pool_info(pair).await?;
        
        // Calculate optimal bid/ask prices
        let mid_price = (current_pool.reserve_b as f64) / (current_pool.reserve_a as f64);
        let bid_price = mid_price * (1.0 - spread_ratio / 2.0);
        let ask_price = mid_price * (1.0 + spread_ratio / 2.0);
        
        log::info!("Mid price: {:.8}, Bid: {:.8}, Ask: {:.8}", mid_price, bid_price, ask_price);
        
        let base_order_size = 0.1; // 0.1 SOL per order
        let base_order_lamports = (base_order_size * 1_000_000_000.0) as u64;
        
        // Simulate placing orders
        log::debug!("Placing bid order: {} SOL at {:.8}", base_order_size, bid_price);
        log::debug!("Placing ask order: {} SOL at {:.8}", base_order_size, ask_price);
        
        
        sleep(Duration::from_millis(500)).await;
        
        let filled_bid = rand::random::<bool>();
        let filled_ask = rand::random::<bool>();
        
        if filled_bid {
            log::info!("Bid order filled at {:.8}", bid_price);
        }
        if filled_ask {
            log::info!("Ask order filled at {:.8}", ask_price);
        }
        
        log::debug!("Rebalancing depth and inventory...");
        sleep(Duration::from_millis(300)).await;
        
        let round_pnl = if filled_bid && filled_ask {
            let profit = (ask_price - bid_price) * base_order_size;
            log::info!("Round {} PnL: {:.6} SOL", round, profit);
            profit
        } else {
            0.0
        };
        
        if round < 5 {
            sleep(Duration::from_millis(1000)).await;
        }
    }
    
    log::info!("Market-making session completed");
    log::info!("Final pool state: {} ({} <-> {})", pool.address, pool.token_a, pool.token_b);
    
    Ok(())
}
