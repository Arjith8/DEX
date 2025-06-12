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

    let client = RpcClient::new("https://api.devnet.solana.com");
    let pubkey = public_key.parse::<Pubkey>().expect("pub key conversion failed");
    let balance = client.get_balance(&pubkey).expect("Failed to get balance");
    println!("{} is your balance", balance)
}

