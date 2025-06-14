use std::env;

use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Debug)]
pub struct Account;

impl Account{
    pub fn get_balance() -> u64 {
        if dotenv().is_err(){
            println!("Failed to extract env vars");
        }

        let public_key = env::var("PUBLIC_KEY").expect("Env var not found");

        let solana_devnet = env::var("SOLANA_DEVNET").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        let client = RpcClient::new(solana_devnet);

        let pubkey = public_key.parse::<Pubkey>().expect("pub key conversion failed");
        let balance = client.get_balance(&pubkey).expect("Failed to get balance");

        balance
    }

    pub fn get_keypair() -> Keypair {
        dotenv().expect("Failed to load .env file");
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env file");
    
        let keypair = Keypair::from_base58_string(&private_key);
        keypair
    }
}
