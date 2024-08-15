use std::error::Error;

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

fn main() {
    let generated_keypair = generate_keypair();
    println!("✅ Generated keypair!");
    println!("Public Key: {}", generated_keypair.pubkey());

    let loaded_keypair = load_keypair();

    println!("✅ Loaded keypair!");
    println!("Public Key: {}", loaded_keypair.pubkey());

    let balance = check_balance(&loaded_keypair.pubkey().to_string());
    match balance {
        Ok(balance) => println!("Balance: {} SOL", balance),
        Err(e) => eprintln!("Error: {}", e),
    };

    airdrop_if_required(&loaded_keypair.pubkey()).unwrap();

    let balance = check_balance(&loaded_keypair.pubkey().to_string());
    match balance {
        Ok(balance) => println!("Balance: {} SOL", balance),
        Err(e) => eprintln!("Error: {}", e),
    };
}

fn generate_keypair() -> Keypair {
    let keypair = Keypair::new();

    keypair
}

fn load_keypair() -> Keypair {
    dotenv().ok();

    let private_key = std::env::var("SECRET_KEY").expect("SECRET_KEY is not set");
    let key_bytes: Vec<u8> =
        serde_json::from_str(&private_key).expect("Failed to parse SECRET_KEY from JSON");
    let keypair = Keypair::from_bytes(&key_bytes).expect("Invalid secret key");

    keypair
}

fn check_balance(pubkey: &str) -> Result<f64, Box<dyn Error>> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = pubkey.parse()?;
    let balance = rpc_client.get_balance(&pubkey)?;

    Ok(balance as f64 / 10f64.powf(9.0))
}

fn airdrop_if_required(public_key: &Pubkey) -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let balance = rpc_client.get_balance(public_key)?;

    if balance < 1 {
        println!(
            "🔄 Current balance is low ({:.2} SOL). Airdropping...",
            balance as f64 / LAMPORTS_PER_SOL as f64
        );

        let signature = rpc_client.request_airdrop(public_key, 1)?;
        rpc_client.confirm_transaction(&signature)?;

        println!("✅ Airdrop successful!");
    } else {
        println!(
            "💰 Sufficient balance ({:.2} SOL). No airdrop required.",
            balance as f64 / LAMPORTS_PER_SOL as f64
        );
    }

    Ok(())
}

#[cfg(test)]
mod solana_tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let keypair = generate_keypair();
        assert_eq!(keypair.pubkey().to_string().len(), 44);
    }

    #[test]
    fn test_load_keypair() {
        let keypair = load_keypair();
        assert_eq!(keypair.pubkey().to_string().len(), 44);
    }

    #[test]
    fn test_check_balance() {
        let keypair = load_keypair();
        let pubkey = keypair.pubkey().to_string();
        let balance = check_balance(&pubkey).unwrap();
        assert!(balance > 0.0);
    }

    #[test]
    fn test_airdrop_if_required() {
        let keypair = load_keypair();
        let result = airdrop_if_required(&keypair.pubkey());
        assert!(result.is_ok());
    }
}
