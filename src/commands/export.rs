
use crate::config::Config;
use crate::solana::SolanaClient;
use crate::bagsfm::BagsFmClient;
use anyhow::{Context, Result};
use chrono::{Utc, SecondsFormat};
use csv::WriterBuilder;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct TradeRow {
    time_utc: String,
    wallet: String,
    token: String,
    token_name: String,
    token_symbol: String,
    side: String,
    qty: f64,
    price: f64,
    price_usd: Option<f64>,
    pnl: f64,
    roi_pct: f64,
    hold_minutes: u64,
    transaction_hash: String,
    gas_fee: f64,
    pool_address: Option<String>,
    dex_type: Option<String>,
}

#[derive(Serialize)]
struct WalletActivityRow {
    time_utc: String,
    wallet: String,
    event_type: String,
    token_address: Option<String>,
    amount_sol: f64,
    transaction_hash: String,
    block_number: u64,
    fee_paid: f64,
}

pub async fn run(cfg: &Config, out_path: &str) -> Result<()> {
    let out = Path::new(out_path);
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).ok();
    }

    // Initialize clients for real data
    let solana_client = SolanaClient::new(&cfg.rpc_url, &cfg.wallet_path).ok();
    let bagsfm_client = BagsFmClient::new();

    // Export trades
    let trades_file = out.with_file_name("trades.csv");
    let mut trades_wtr = WriterBuilder::new().from_path(&trades_file).context("failed to open trades CSV")?;
    
    // Export wallet activity
    let activity_file = out.with_file_name("wallet_activity.csv");
    let mut activity_wtr = WriterBuilder::new().from_path(&activity_file).context("failed to open activity CSV")?;

    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    //this is demo data
    let trades = vec![
        TradeRow { 
            time_utc: now.clone(), 
            wallet: cfg.wallet_address.clone(), 
            token: "DemoToken123".into(), 
            token_name: "Demo Token Alpha".into(),
            token_symbol: "DEMO1".into(),
            side: "BUY".into(), 
            qty: 1000.0, 
            price: 0.0021, 
            price_usd: Some(0.21),
            pnl: 0.0, 
            roi_pct: 0.0, 
            hold_minutes: 0,
            transaction_hash: "DemoTxHash123".into(),
            gas_fee: 0.000005,
            pool_address: Some("Pool123".into()),
            dex_type: Some("Raydium".into()),
        },
        TradeRow { 
            time_utc: now.clone(), 
            wallet: cfg.wallet_address.clone(), 
            token: "DemoToken123".into(), 
            token_name: "Demo Token Alpha".into(),
            token_symbol: "DEMO1".into(),
            side: "SELL".into(), 
            qty: 1000.0, 
            price: 0.0030, 
            price_usd: Some(0.30),
            pnl: 0.9, 
            roi_pct: 42.85, 
            hold_minutes: 37,
            transaction_hash: "DemoTxHash456".into(),
            gas_fee: 0.000005,
            pool_address: Some("Pool123".into()),
            dex_type: Some("Raydium".into()),
        },
        TradeRow { 
            time_utc: now.clone(), 
            wallet: cfg.wallet_address.clone(), 
            token: "DemoToken456".into(), 
            token_name: "Demo Token Beta".into(),
            token_symbol: "DEMO2".into(),
            side: "BUY".into(), 
            qty: 500.0, 
            price: 0.0015, 
            price_usd: Some(0.15),
            pnl: 0.0, 
            roi_pct: 0.0, 
            hold_minutes: 0,
            transaction_hash: "DemoTxHash789".into(),
            gas_fee: 0.000005,
            pool_address: Some("Pool456".into()),
            dex_type: Some("Orca".into()),
        },
    ];

    let activities = vec![
        WalletActivityRow {
            time_utc: now.clone(),
            wallet: cfg.wallet_address.clone(),
            event_type: "SWAP".into(),
            token_address: Some("DemoToken123".into()),
            amount_sol: 0.5,
            transaction_hash: "DemoTxHash123".into(),
            block_number: 123456789,
            fee_paid: 0.000005,
        },
        WalletActivityRow {
            time_utc: now.clone(),
            wallet: cfg.wallet_address.clone(),
            event_type: "LP_ADD".into(),
            token_address: Some("DemoToken456".into()),
            amount_sol: 1.0,
            transaction_hash: "DemoTxHash456".into(),
            block_number: 123456790,
            fee_paid: 0.000005,
        },
        WalletActivityRow {
            time_utc: now.clone(),
            wallet: cfg.wallet_address.clone(),
            event_type: "TRANSFER".into(),
            token_address: None,
            amount_sol: 0.1,
            transaction_hash: "DemoTxHash789".into(),
            block_number: 123456791,
            fee_paid: 0.000005,
        },
    ];

    for trade in trades {
        trades_wtr.serialize(trade)?;
    }
    trades_wtr.flush()?;

    for activity in activities {
        activity_wtr.serialize(activity)?;
    }
    activity_wtr.flush()?;
    let summary_file = out.with_file_name("summary.txt");
    let summary_content = format!(
        "ReoswellEcho Trading Bot Export Summary
Generated: {}
Wallet: {}
Total Trades: {}
Total Volume: {:.4} SOL
Average Trade Size: {:.4} SOL
Export Directory: {}

Configuration:
- RPC: {}
- Slippage: {} bps
- Max Retries: {}
- Budget Cap: {:.4} SOL
- Buy Cap: {:.4} SOL

Files Generated:
- trades.csv: Detailed trade history
- wallet_activity.csv: Wallet transaction activity
- summary.txt: This summary report

Note: This is a development export with sample data.
Real implementation would include actual blockchain transaction data.",
        now,
        cfg.wallet_address,
        trades.len(),
        2.1, 
        0.7, 
        cfg.export_dir,
        cfg.rpc_url,
        cfg.slippage_bps,
        cfg.max_retries,
        cfg.budget_sol,
        cfg.buy_cap_sol
    );

    fs::write(&summary_file, summary_content)?;

    log::info!("Export completed successfully!");
    log::info!("Trades CSV: {}", trades_file.display());
    log::info!("Activity CSV: {}", activity_file.display());
    log::info!("Summary: {}", summary_file.display());

    Ok(())
}
