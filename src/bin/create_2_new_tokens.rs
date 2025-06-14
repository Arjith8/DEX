use std::{env, fs};

use dex::utils::account::{MainAccount, MintAccount};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, program_pack::Pack, signer::Signer, system_instruction::create_account, transaction::Transaction};
use spl_token_2022::{instruction::initialize_mint, state::Mint};

fn main() {
    let balance = MainAccount::get_balance();
    println!("You have {} lamports of Solana", balance);

    println!("Creating 2 new tokens...");
    let solana_devnet = env::var("SOLANA_DEVNET").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let client = RpcClient::new_with_commitment(solana_devnet, CommitmentConfig::confirmed());

    let latest_block_hash = client.get_latest_blockhash().expect("Failed to get latest block hash");

    let fee_payer = MainAccount::get_keypair();

    let mint_space = Mint::LEN;
    println!("Mint space: {}", mint_space);

    println!("Blockhash: {:?}", latest_block_hash);

    let min_balance_for_rent_excemption = client.get_minimum_balance_for_rent_exemption(mint_space)
        .expect("Failed to get minimum balance for rent exemption");

    println!("Minimum balance for rent exemption: {}", min_balance_for_rent_excemption);
        
    let mint_keypair = MintAccount::get_keypair();

    let create_account_instruction = create_account(
        &fee_payer.pubkey(),
        &mint_keypair.pubkey(),
        min_balance_for_rent_excemption,
        mint_space as u64,
        &spl_token_2022::id(),
    );

    let token_min_instructions = initialize_mint(&spl_token_2022::id(), &mint_keypair.pubkey(), &fee_payer.pubkey(), Some(&fee_payer.pubkey()), 2)
        .expect("Failed to create token mint instruction");

    let transaction = Transaction::new_signed_with_payer(
            &[create_account_instruction, token_min_instructions], 
            Some(&fee_payer.pubkey()), 
            &[&fee_payer, &mint_keypair], 
            latest_block_hash
        );

    let transaction_signature = client.send_and_confirm_transaction(&transaction)
        .expect("Failed to send and confirm transaction");

    println!("Transaction signature: {}", transaction_signature);
}
