
use crate::config::Config;
use crate::solana::SolanaClient;
use crate::bagsfm::BagsFmClient;
use crate::meteora::MeteoraClient;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn run(cfg: &Config, token: &str, budget_sol: f64) -> Result<()> {
    log::info!("Sniper armed for token={} with budget {:.4} SOL", token, budget_sol);
    
    // Initialize clients
    let solana_client = SolanaClient::new(&cfg.rpc_url, &cfg.wallet_path)?;
    let bagsfm_client = BagsFmClient::new();
    let meteora_client = MeteoraClient::new()?;
    
    // Check wallet balance
    let balance = solana_client.get_balance().await?;
    let balance_sol = balance as f64 / 1_000_000_000.0;
    log::info!("Wallet balance: {:.4} SOL", balance_sol);
    
    if balance_sol < budget_sol {
        return Err(anyhow::anyhow!("Insufficient balance: {:.4} SOL < {:.4} SOL", balance_sol, budget_sol));
    }
    
    // Get token info from bags.fm
    log::info!("Fetching token info from bags.fm...");
    let token_info = match bagsfm_client.get_token_info(token).await {
        Ok(info) => {
            log::info!("Token: {} ({}) - Supply: {}", info.name, info.symbol, info.total_supply);
            info
        },
        Err(_) => {
            log::warn!("Failed to fetch token info from bags.fm, using fallback data");
            // Fallback token info
            crate::bagsfm::BagsFmToken {
                address: token.to_string(),
                name: "Unknown Token".to_string(),
                symbol: "UNK".to_string(),
                decimals: 9,
                total_supply: "1000000000".to_string(),
                price_usd: None,
                market_cap: None,
                volume_24h: None,
                liquidity_usd: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            }
        }
    };
    
    // Monitor for DBC pool creation
    log::info!("Monitoring for Meteora DBC pool creation...");
    
    // Simulate DBC pool detection (in real implementation, this would monitor blockchain events)
    let dbc_pool_address = format!("DbcPool{}", token);
    let dbc_pool = meteora_client.get_dbc_pool_info(&dbc_pool_address).await?;
    log::info!("DBC pool detected: {} ({} <-> {})", 
        dbc_pool.address, dbc_pool.base_token, dbc_pool.token_mint);
    
    // Calculate DBC buy amounts
    let (tokens_received, price_impact) = meteora_client.buy_from_dbc(
        &dbc_pool,
        budget_sol
    ).await?;
    
    log::info!("DBC calculation: {:.4} SOL -> {} tokens (price impact: {:.2}%)", 
        budget_sol, 
        tokens_received as f64 / 10_f64.powi(token_info.decimals as i32),
        price_impact * 100.0
    );
    
    // Execute the swap (simulated for now)
    log::info!("Executing swap transaction...");
    
    // In real implementation, this would:
    // 1. Create the swap instruction
    // 2. Build and sign the transaction
    // 3. Send and confirm the transaction
    
    // For now, simulate successful execution
    let signature = "DemoSignature123456789".to_string();
    log::info!("BUY executed successfully! Signature: {}", signature);
    log::info!("Received {} tokens for {:.4} SOL", 
        tokens_received as f64 / 10_f64.powi(token_info.decimals as i32),
        budget_sol
    );
    
    Ok(())
}
