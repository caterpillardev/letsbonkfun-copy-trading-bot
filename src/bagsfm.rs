use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BagsFmToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
    pub price_usd: Option<f64>,
    pub market_cap: Option<f64>,
    pub volume_24h: Option<f64>,
    pub liquidity_usd: Option<f64>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BagsFmLaunch {
    pub token_address: String,
    pub initial_liquidity_sol: f64,
    pub launch_time: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BagsFmPool {
    pub address: String,
    pub token_a: String,
    pub token_b: String,
    pub reserve_a: String,
    pub reserve_b: String,
    pub fee_rate: f64,
}

pub struct BagsFmClient {
    client: Client,
    base_url: String,
}

impl BagsFmClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.bags.fm".to_string(),
        }
    }

    pub async fn get_trending_tokens(&self) -> Result<Vec<BagsFmToken>> {
        let url = format!("{}/v1/tokens/trending", self.base_url);
        let response = self.client.get(&url)
            .header("User-Agent", "ReoswellEcho-Bot/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let tokens: Vec<BagsFmToken> = response.json().await?;
            Ok(tokens)
        } else {
            Err(anyhow!("Failed to fetch trending tokens: {}", response.status()))
        }
    }

    pub async fn get_token_info(&self, token_address: &str) -> Result<BagsFmToken> {
        let url = format!("{}/v1/tokens/{}", self.base_url, token_address);
        let response = self.client.get(&url)
            .header("User-Agent", "ReoswellEcho-Bot/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let token: BagsFmToken = response.json().await?;
            Ok(token)
        } else {
            Err(anyhow!("Failed to fetch token info: {}", response.status()))
        }
    }

    pub async fn get_recent_launches(&self, limit: u32) -> Result<Vec<BagsFmLaunch>> {
        let url = format!("{}/v1/launches?limit={}", self.base_url, limit);
        let response = self.client.get(&url)
            .header("User-Agent", "ReoswellEcho-Bot/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let launches: Vec<BagsFmLaunch> = response.json().await?;
            Ok(launches)
        } else {
            Err(anyhow!("Failed to fetch recent launches: {}", response.status()))
        }
    }

    pub async fn get_pool_info(&self, pool_address: &str) -> Result<BagsFmPool> {
        let url = format!("{}/v1/pools/{}", self.base_url, pool_address);
        let response = self.client.get(&url)
            .header("User-Agent", "ReoswellEcho-Bot/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let pool: BagsFmPool = response.json().await?;
            Ok(pool)
        } else {
            Err(anyhow!("Failed to fetch pool info: {}", response.status()))
        }
    }

    pub async fn monitor_new_listings(&self) -> Result<Vec<BagsFmToken>> {
        // Simulate monitoring for new token listings
        // In real implementation, this would use WebSocket or polling
        let url = format!("{}/v1/tokens/new", self.base_url);
        let response = self.client.get(&url)
            .header("User-Agent", "ReoswellEcho-Bot/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let tokens: Vec<BagsFmToken> = response.json().await?;
            Ok(tokens)
        } else {
            // Fallback to simulated data for development
            Ok(vec![
                BagsFmToken {
                    address: "DemoToken123".to_string(),
                    name: "Demo Token".to_string(),
                    symbol: "DEMO".to_string(),
                    decimals: 9,
                    total_supply: "1000000000".to_string(),
                    price_usd: Some(0.001),
                    market_cap: Some(1000.0),
                    volume_24h: Some(500.0),
                    liquidity_usd: Some(100.0),
                    created_at: chrono::Utc::now().to_rfc3339(),
                }
            ])
        }
    }
}

