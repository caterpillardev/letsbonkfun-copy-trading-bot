
use clap::{Parser, Subcommand};
use anyhow::Result;

mod config;
mod commands;
mod solana;
mod bagsfm;
mod dex;
mod meteora;

use commands::{bundler, sniper, market_maker, tracker, export};

#[derive(Parser, Debug)]
#[command(author, version, about = "ReoswellEcho — bags.fm trading weapon (bundler/sniper/mm/track/export)", long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Bundler {
        #[arg(short, long, default_value = "plans/example.json")]
        plan: String,
        #[arg(long)]
        dry_run: bool,
    },
    Sniper {
        #[arg(short, long)]
        token: String,
        #[arg(long, default_value_t = 1.0)]
        budget_sol: f64,
    },
    MarketMaker {
        #[arg(short, long)]
        pair: String,
        #[arg(long, default_value_t = 100)]
        spread_bps: u32,
    },
    Track {
        #[arg(short, long)]
        wallet: String,
    },
    Export {
        #[arg(short, long, default_value = "out/trades.csv")]
        out: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let mut builder = env_logger::Builder::from_default_env();
    if cli.verbose > 0 {
        builder.filter_level(log::LevelFilter::Debug);
    }
    builder.init();

    let cfg = config::Config::from_env()?;
    log::info!("Loaded config: {:?}", cfg.safe());

    match cli.command {
        Commands::Bundler { plan, dry_run } => bundler::run(&cfg, &plan, dry_run).await?,
        Commands::Sniper { token, budget_sol } => sniper::run(&cfg, &token, budget_sol).await?,
        Commands::MarketMaker { pair, spread_bps } => market_maker::run(&cfg, &pair, spread_bps).await?,
        Commands::Track { wallet } => tracker::run(&cfg, &wallet).await?,
        Commands::Export { out } => export::run(&cfg, &out).await?,
    }

    Ok(())
}
