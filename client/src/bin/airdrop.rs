use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use std::env;

fn main() {
    if dotenv().is_err() {
        println!("Failed to extract env vars");
        return;
    }

    let solana_devnet = env::var("SOLANA_DEVNET").expect("Solana DevNet value not found");

    let airdrop_reciever_public_key =
        env::var("PUBLIC_KEY").expect("Public key env variable not found");
    let pubkey = airdrop_reciever_public_key
        .parse::<Pubkey>()
        .expect("Invalid Public Key, Conversion Failed");

    let client = RpcClient::new(solana_devnet);

    let signature = client
        .request_airdrop(&pubkey, LAMPORTS_PER_SOL * 5)
        .unwrap();

    let confirmation = client.confirm_transaction(&signature).unwrap();
    if !confirmation {
        println!("The Airdrop Failed");
        return;
    }

    println!("The DevNet Airdrop is now successful");
}
