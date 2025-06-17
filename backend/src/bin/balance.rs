use dex::utils::{account::Account, account::AccountType};

fn main(){
    let balance = Account::get_balance(AccountType::MAIN, None);

    println!("You have {} lamports of Solana", balance)
}

