use solana_client::rpc_client::RpcClient;
use dotenvy::dotenv;
use solana_sdk::pubkey::Pubkey;
use std::env;

fn main(){
    if dotenv().is_err(){
        println!("Failed to extract env vars");
        return;
    }

    let public_key = env::var("PUBLIC_KEY").expect("Env var not found");

    let solana_devnet = env::var("SOLANA_DEVNET").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let client = RpcClient::new(solana_devnet);
    let pubkey = public_key.parse::<Pubkey>().expect("pub key conversion failed");
    let balance = client.get_balance(&pubkey).expect("Failed to get balance");
    println!("You have {} lamports of Solana", balance)
}

