use std::{env, fs, path::Path};

use dotenvy::dotenv;
use nanoid::nanoid;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use strum_macros::{Display, EnumString};

pub struct Account;
#[derive(Display, PartialEq, EnumString, Debug)]
pub enum AccountType {
    #[strum(serialize = "mint")]
    MINT,

    #[strum(serialize = "main")]
    MAIN,

    #[strum(serialize = "vault")]
    VAULT,

    #[strum(serialize = "lp")]
    LP,
}

impl Account {
    pub fn get_keypair(account_type: AccountType, key_path: Option<&String>) -> Keypair {
        let keypair: Keypair;
        if account_type == AccountType::MAIN {
            dotenv().expect("Failed to load .env file");
            let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env file");

            keypair = Keypair::from_base58_string(&private_key);
        } else {
            let key_path = key_path.expect("You have to provide keypair path for non main accounts (account mentioned in env var)");
            let path = Path::new(key_path);
            let private_key = fs::read_to_string(path).expect("Failed to read keypair file");
            keypair = Keypair::from_base58_string(&private_key);
        }
        keypair
    }

    pub fn get_balance(account_type: AccountType, key_path: Option<&String>) -> u64 {
        if dotenv().is_err() {
            println!("Failed to extract env vars");
        }

        let solana_devnet = env::var("SOLANA_DEVNET")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        let client = RpcClient::new(solana_devnet);

        let pubkey: Pubkey;
        if account_type == AccountType::MAIN {
            let public_key = env::var("PUBLIC_KEY").expect("Env var not found");
            pubkey = public_key
                .parse::<Pubkey>()
                .expect("pub key conversion failed");
        } else {
            pubkey = Account::get_keypair(account_type, key_path).pubkey();
        }

        let balance = client.get_balance(&pubkey).expect("Failed to get balance");

        balance
    }

    pub fn create_account(account_type: AccountType) {
        dotenv().expect("Failed to load environment variables from .env file");

        let keypair = Keypair::new().to_base58_string();

        let keys_dir = Path::new("./keys");
        if fs::exists(keys_dir).is_ok_and(|exists| !exists) {
            fs::create_dir(keys_dir).expect("Failed to create keys directory");
        };

        let id = nanoid!(10);
        let filename = format!("{}_keypair{}", account_type, id);

        let path = keys_dir.join(filename);

        if path.exists() {
            println!("Keypair file already exists at: {}", path.display());
            println!("Aborting this operation to avoid overwriting existing keypair");
            return;
        }
        fs::write(path, &keypair).expect("Failed to write keypair to file");

        println!("{} keypair has been successfully created", account_type);
    }
}
