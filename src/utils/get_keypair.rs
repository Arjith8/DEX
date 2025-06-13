use std::env;

use dotenvy::dotenv;
use solana_sdk::signature::Keypair;

pub fn get_keypair() {
    dotenv().expect("Failed to load .env file");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env file");

    //let decoded_private_key = bs58::decode(&private_key);

    let keypair = Keypair::from_base58_string(&private_key);
    println!("Keypair: {:?}", keypair);
}
