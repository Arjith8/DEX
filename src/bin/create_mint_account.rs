use std::{fs, path::Path};

use dotenvy::dotenv;
use solana_sdk::{signature::Keypair};

fn main() {
    dotenv()
        .expect("Failed to load environment variables from .env file");

    let mint_keypair = Keypair::new().to_base58_string();

    let keys_dir = Path::new("./keys");
    if fs::exists(keys_dir).is_ok_and(|exists| !exists) {
        fs::create_dir(keys_dir).expect("Failed to create keys directory");
    }

    let path = Path::new("./keys/mint_keypair");

    fs::write(path, &mint_keypair).expect("Failed to write mint keypair to file");
        
    println!("Mint Account Public Key: {}", mint_keypair);
}
