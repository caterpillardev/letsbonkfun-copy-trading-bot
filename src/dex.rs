use anyhow::{anyhow, Result};
use solana_sdk::{
    pubkey::Pubkey,
    instruction::Instruction,
    transaction::Transaction,
    system_instruction,
};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DexPool {
    pub address: String,
    pub token_a: String,
    pub token_b: String,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_rate: f64,
    pub dex_type: DexType,
}

#[derive(Debug, Clone)]
pub enum DexType {
    Raydium,
    Orca,
}

#[derive(Debug)]
pub struct SwapResult {
    pub signature: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub fee_paid: u64,
    pub price_impact: f64,
}

pub struct DexClient {
    raydium_program_id: Pubkey,
    orca_program_id: Pubkey,
}

impl DexClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            raydium_program_id: Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")?,
            orca_program_id: Pubkey::from_str("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM")?,
        })
    }

    pub async fn get_pool_info(&self, pool_address: &str) -> Result<DexPool> {
        let pool = DexPool {
            address: pool_address.to_string(),
            token_a: "So11111111111111111111111111111111111111112".to_string(), // SOL
            token_b: "DemoToken123".to_string(),
            reserve_a: 100_000_000_000, 
            reserve_b: 1_000_000_000_000,
            fee_rate: 0.0025, 
            dex_type: DexType::Raydium,
        };
        Ok(pool)
    }

    pub async fn calculate_swap_amounts(
        &self,
        pool: &DexPool,
        input_amount: u64,
        is_input_a: bool,
    ) -> Result<(u64, u64)> {
        let (reserve_in, reserve_out) = if is_input_a {
            (pool.reserve_a, pool.reserve_b)
        } else {
            (pool.reserve_b, pool.reserve_a)
        };

        let fee_multiplier = 1.0 - pool.fee_rate;
        let input_with_fee = (input_amount as f64) * fee_multiplier;
        
        let output_amount = (reserve_out as f64 * input_with_fee) / 
                           (reserve_in as f64 + input_with_fee);
        
        let fee_paid = input_amount - (input_with_fee as u64);
        
        Ok((output_amount as u64, fee_paid))
    }

    pub async fn create_swap_instruction(
        &self,
        pool: &DexPool,
        user_wallet: &Pubkey,
        input_mint: &str,
        output_mint: &str,
        input_amount: u64,
        min_output_amount: u64,
    ) -> Result<Instruction> {
        match pool.dex_type {
            DexType::Raydium => self.create_raydium_swap_instruction(
                pool, user_wallet, input_mint, output_mint, input_amount, min_output_amount
            ).await,
            DexType::Orca => self.create_orca_swap_instruction(
                pool, user_wallet, input_mint, output_mint, input_amount, min_output_amount
            ).await,
        }
    }

    async fn create_raydium_swap_instruction(
        &self,
        _pool: &DexPool,
        _user_wallet: &Pubkey,
        _input_mint: &str,
        _output_mint: &str,
        _input_amount: u64,
        _min_output_amount: u64,
    ) -> Result<Instruction> {
        //Instruction
        Err(anyhow!("Raydium swap instruction not yet implemented"))
    }

    async fn create_orca_swap_instruction(
        &self,
        _pool: &DexPool,
        _user_wallet: &Pubkey,
        _input_mint: &str,
        _output_mint: &str,
        _input_amount: u64,
        _min_output_amount: u64,
    ) -> Result<Instruction> {
        Err(anyhow!("Orca swap instruction not yet implemented"))
    }

    pub async fn add_liquidity(
        &self,
        pool: &DexPool,
        token_a_amount: u64,
        token_b_amount: u64,
    ) -> Result<u64> {
        let total_lp_supply = 1_000_000_000; 
        
        let lp_tokens = if pool.reserve_a == 0 && pool.reserve_b == 0 {
            ((token_a_amount as f64 * token_b_amount as f64).sqrt() as u64).max(1_000_000)
        } else {
            let ratio_a = (token_a_amount as f64) / (pool.reserve_a as f64);
            let ratio_b = (token_b_amount as f64) / (pool.reserve_b as f64);
            let ratio = ratio_a.min(ratio_b);
            (total_lp_supply as f64 * ratio) as u64
        };
        
        Ok(lp_tokens)
    }

    pub async fn remove_liquidity(
        &self,
        pool: &DexPool,
        lp_tokens: u64,
    ) -> Result<(u64, u64)> {
        let total_lp_supply = 1_000_000_000; 
        let ratio = (lp_tokens as f64) / (total_lp_supply as f64);
        
        let token_a_amount = (pool.reserve_a as f64 * ratio) as u64;
        let token_b_amount = (pool.reserve_b as f64 * ratio) as u64;
        
        Ok((token_a_amount, token_b_amount))
    }
}

