use std::{env, error::Error, str::FromStr};

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
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

    let recipient = "EtvBVHgsoJMuwd6kSFuoMEdRY8EJ67G2J6q7DxVRT5pt";
    let send_transaction_result = send_transaction(recipient);
    match send_transaction_result {
        Ok(signature) => println!("✅ Transaction sent! Signature: {}", signature),
        Err(e) => eprintln!("Error: {}", e),
    };

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

    let private_key = env::var("SECRET_KEY").expect("SECRET_KEY is not set");
    let key_bytes: Vec<u8> =
        serde_json::from_str(&private_key).expect("Failed to parse SECRET_KEY from JSON");
    let keypair = Keypair::from_bytes(&key_bytes).expect("Invalid secret key");

    keypair
}

fn check_balance(pubkey: &str) -> Result<f64, Box<dyn Error>> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = pubkey.parse().expect("Invalid public key");
    let balance = rpc_client
        .get_balance(&pubkey)
        .expect("Failed to get balance");

    Ok(balance as f64 / 10f64.powf(9.0))
}

fn airdrop_if_required(public_key: &Pubkey) -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let balance = rpc_client
        .get_balance(public_key)
        .expect("Failed to get balance");

    if balance < 1 {
        println!(
            "🔄 Current balance is low ({:.2} SOL). Airdropping...",
            balance as f64 / LAMPORTS_PER_SOL as f64
        );

        let signature = rpc_client
            .request_airdrop(public_key, 1)
            .expect("Airdrop failed");
        rpc_client
            .confirm_transaction(&signature)
            .expect("Transaction failed");

        println!("✅ Airdrop successful!");
    } else {
        println!(
            "💰 Sufficient balance ({:.2} SOL). No airdrop required.",
            balance as f64 / LAMPORTS_PER_SOL as f64
        );
    }

    Ok(())
}

fn send_transaction(recipient: &str) -> Result<solana_sdk::signature::Signature, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let private_key = env::var("SECRET_KEY").expect("SECRET_KEY is not set");
    let key_bytes: Vec<u8> =
        serde_json::from_str(&private_key).expect("Failed to parse SECRET_KEY from JSON");
    let sender = Keypair::from_bytes(&key_bytes).expect("Invalid secret key");
    let recipient_pub_key = Pubkey::from_str(recipient).expect("Incorrect recipient public key");

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    println!("🔑 Our public key is: {}", sender.pubkey());

    let amount_in_lamports = (0.01 * LAMPORTS_PER_SOL as f64) as u64;
    let transfer_instruction =
        system_instruction::transfer(&sender.pubkey(), &recipient_pub_key, amount_in_lamports);

    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")
        .expect("Invalid program id");
    let memo_text = "Hello from Solana!";
    let memo_instruction = Instruction {
        program_id: memo_program,
        accounts: vec![],
        data: memo_text.as_bytes().to_vec(),
    };

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let message = Message::new(
        &[transfer_instruction, memo_instruction],
        Some(&sender.pubkey()),
    );
    let mut transaction = Transaction::new_unsigned(message);
    transaction
        .try_sign(&[&sender], recent_blockhash)
        .expect("Failed to sign transaction");
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");

    Ok(signature)
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

    #[test]
    fn test_send_transaction() {
        let recipient = "EtvBVHgsoJMuwd6kSFuoMEdRY8EJ67G2J6q7DxVRT5pt";
        let result = send_transaction(recipient);
        assert!(result.is_ok());
    }
}
