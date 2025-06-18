use std::env::args;

use dex::utils::{account::Account, account::AccountType};

fn main() {
    let user_provided_args: Vec<String> = args().into_iter().collect();
    println!("User provided args: {:?}", user_provided_args);
    let account_type = &user_provided_args[1];
    let balance: u64;
    if account_type.to_lowercase() == "main" {
        balance = Account::get_balance(AccountType::MAIN, None);
    } else {
        if user_provided_args.len() < 3 {
            panic!(
                "You have to provide keypair path for non main accounts (account mentioned in env var)"
            );
        }
        let keypair_path = &user_provided_args[2];
        if account_type.to_lowercase() == "mint" {
            balance = Account::get_balance(AccountType::MINT, Some(keypair_path));
        } else if account_type.to_lowercase() == "vault" {
            balance = Account::get_balance(AccountType::VAULT, Some(keypair_path));
        } else {
            panic!("Invalid account type provided. Use 'main', 'mint', or 'vault'.");
        }
    }
    println!("You have {} lamports of Solana", balance)
}
