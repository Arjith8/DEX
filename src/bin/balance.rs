use dex::utils;

fn main(){
    let balance = utils::account::Account::get_balance();

    println!("You have {} lamports of Solana", balance)
}

