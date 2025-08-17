use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, read_keypair_file},
    transaction::Transaction,
    system_instruction,
};
use solana_transaction_status::UiTransactionEncoding;
use spl_token::state::Account as TokenAccount;
use std::str::FromStr;

pub struct SolanaClient {
    pub rpc_client: RpcClient,
    pub wallet: Keypair,
    pub wallet_pubkey: Pubkey,
}

impl SolanaClient {
    pub fn new(rpc_url: &str, wallet_path: &str) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed()
        );
        
        let wallet = read_keypair_file(wallet_path)
            .map_err(|e| anyhow!("Failed to read wallet: {}", e))?;
        let wallet_pubkey = wallet.pubkey();
        
        Ok(Self {
            rpc_client,
            wallet,
            wallet_pubkey,
        })
    }

    pub async fn get_balance(&self) -> Result<u64> {
        let balance = self.rpc_client.get_balance(&self.wallet_pubkey)?;
        Ok(balance)
    }

    pub async fn get_token_balance(&self, token_mint: &str) -> Result<u64> {
        let mint_pubkey = Pubkey::from_str(token_mint)?;
        let token_accounts = self.rpc_client.get_token_accounts_by_owner(
            &self.wallet_pubkey,
            spl_token::instruction::TokenAccountFilter::Mint(mint_pubkey),
        )?;

        if let Some(account) = token_accounts.first() {
            let account_data = base64::decode(&account.account.data[0])?;
            let token_account = TokenAccount::unpack(&account_data)?;
            Ok(token_account.amount)
        } else {
            Ok(0)
        }
    }

    pub async fn send_sol(&self, to: &str, amount_sol: f64) -> Result<String> {
        let to_pubkey = Pubkey::from_str(to)?;
        let amount_lamports = (amount_sol * 1_000_000_000.0) as u64;
        
        let instruction = system_instruction::transfer(
            &self.wallet_pubkey,
            &to_pubkey,
            amount_lamports,
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.wallet_pubkey),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }

    pub async fn get_transaction(&self, signature: &str) -> Result<solana_transaction_status::EncodedConfirmedTransaction> {
        let sig = bs58::decode(signature).into_vec()?;
        let transaction = self.rpc_client.get_transaction(
            &sig.try_into()?,
            UiTransactionEncoding::Json,
            solana_transaction_status::UiTransactionConfig {
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )?;
        Ok(transaction)
    }
}

