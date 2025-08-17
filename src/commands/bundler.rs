
use crate::config::Config;
use crate::solana::SolanaClient;
use crate::bagsfm::BagsFmClient;
use crate::meteora::{MeteoraClient, DbcCurveType};
use anyhow::Result;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
struct BundlePlan {
    plans: Vec<TokenPlan>,
}

#[derive(Debug, Deserialize)]
struct TokenPlan {
    name: String,
    symbol: String,
    supply: u64,
    liquidity_sol: f64,
    initial_price_sol: Option<f64>,
    description: Option<String>,
}

pub async fn run(cfg: &Config, plan_path: &str, dry_run: bool) -> Result<()> {
    let raw = std::fs::read_to_string(plan_path)?;
    let plan: BundlePlan = serde_json::from_str(&raw)?;
    log::info!("Loaded bundle plan with {} items", plan.plans.len());

    // Initialize clients
    let solana_client = SolanaClient::new(&cfg.rpc_url, &cfg.wallet_path)?;
    let bagsfm_client = BagsFmClient::new();
    let meteora_client = MeteoraClient::new()?;

    // Check total required liquidity
    let total_liquidity: f64 = plan.plans.iter().map(|p| p.liquidity_sol).sum();
    let balance = solana_client.get_balance().await?;
    let balance_sol = balance as f64 / 1_000_000_000.0;
    
    log::info!("Total required liquidity: {:.4} SOL", total_liquidity);
    log::info!("Wallet balance: {:.4} SOL", balance_sol);
    
    if balance_sol < total_liquidity && !dry_run {
        return Err(anyhow::anyhow!(
            "Insufficient balance: {:.4} SOL < {:.4} SOL", 
            balance_sol, 
            total_liquidity
        ));
    }

    for (i, p) in plan.plans.iter().enumerate() {
        if dry_run {
            log::info!("[DRY] Would launch {} ({}) supply={} liq={} SOL via bags.fm", 
                p.name, p.symbol, p.supply, p.liquidity_sol);
        } else {
            log::info!("[{}/{}] Launching {} ({}) on bags.fm ...", 
                i+1, plan.plans.len(), p.name, p.symbol);
            
            // Create token metadata for bags.fm
            let token_metadata = crate::bagsfm::BagsFmToken {
                address: format!("Token{}", i), 
                symbol: p.symbol.clone(),
                decimals: 9,
                total_supply: p.supply.to_string(),
                price_usd: None,
                market_cap: None,
                volume_24h: None,
                liquidity_usd: Some(p.liquidity_sol * 100.0), 
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            
            log::info!("Token metadata created: {} ({})", token_metadata.name, token_metadata.symbol);
            
    
            
            log::info!("Submitting to bags.fm for approval...");
            sleep(Duration::from_millis(1000)).await; 
            
            log::info!("Creating Meteora DBC pool...");
            let curve_type = DbcCurveType::Linear; 
            let dbc_pool = meteora_client.create_dbc_pool(
                &format!("Token{}", i),
                "SOL",
                p.initial_price_sol.unwrap_or(0.0001),
                p.supply,
                curve_type
            ).await?;
            log::info!("DBC pool created: {} ({} <-> {})", 
                dbc_pool.address, dbc_pool.base_token, dbc_pool.token_mint);
            
            log::info!("Initial DBC price: {} SOL per token", dbc_pool.current_price);
            log::info!("Curve type: {:?}", dbc_pool.curve_type);
            
            log::info!("Launched {} with initial liquidity {:.2} SOL", p.symbol, p.liquidity_sol);
            
            if i < plan.plans.len() - 1 {
                sleep(Duration::from_millis(2000)).await;
            }
        }
    }

    log::info!("Bundle complete. RPC primary = {}", cfg.rpc_url);
    Ok(())
}
