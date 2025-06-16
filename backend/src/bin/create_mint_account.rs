use std::{fs, path::Path};

use dotenvy::dotenv;
use solana_sdk::{signature::Keypair};
use nanoid::nanoid;

fn main() {
    dotenv()
        .expect("Failed to load environment variables from .env file");

    let keypair = Keypair::new().to_base58_string();

    let keys_dir = Path::new("./keys");
    if fs::exists(keys_dir).is_ok_and(|exists| !exists) {
        fs::create_dir(keys_dir).expect("Failed to create keys directory");
    };

    let id = nanoid!(10);
    let filename = format!("mint_keypair{}", id);

    let path = keys_dir.join(filename);

    if path.exists() {
        println!("Keypair file already exists at: {}", path.display());
        println!("Aborting this operation to avoid overwriting existing keypair");
        return;
    }
    fs::write(path, &keypair).expect("Failed to write mint keypair to file");
        
    println!("Mint keypair has been successfully created");
}
