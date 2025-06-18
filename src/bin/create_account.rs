use std::env::args;

use dex::utils::account::{Account, AccountType};

fn main() {
    let user_args: Vec<String> = args().collect();
    let account_type = user_args
        .get(1)
        .expect("You have to provide account type as first argument")
        .parse::<AccountType>()
        .expect("Invalid account type provided");

    if account_type == AccountType::MAIN {
        println!("Cannot create main account, it should already be provided in .env file");
        return;
    } else {
        Account::create_account(account_type);
    }
}
