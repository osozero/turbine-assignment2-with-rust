// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod programs;

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

pub fn keygen() -> Keypair {
    Keypair::new()
}

#[cfg(test)]
mod tests {
    use solana_sdk::{self, signature::Keypair, signer::Signer, system_program};
    #[test]
    fn keygen() {
        let keypair = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            keypair.try_pubkey().unwrap()
        );
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", keypair.to_bytes());
    }

    #[test]
    fn base58_to_bytes() {
        let base58 = "kMjLCqPr2dSyndjSaaVQK4RKDK9FhQeRZk9b9HUnbsTRBHePeXaZbM9Ng6ynAjvJzqW93nVweCbKjx6S7M4aiA4";
        let bytes = bs58::decode(base58).into_vec().unwrap();

        let keypair = Keypair::from_bytes(&bytes).unwrap();
        println!("You've loaded a Solana wallet: {:?}", keypair.to_bytes());
    }

    #[test]
    fn bytes_to_base58() {
        let bytes: Vec<u8> = vec![
            155, 66, 127, 84, 33, 237, 245, 69, 202, 137, 115, 203, 154, 63, 46, 227, 250, 157,
            116, 246, 12, 37, 56, 240, 142, 34, 174, 234, 109, 105, 7, 119, 62, 43, 160, 148, 56,
            56, 243, 81, 126, 190, 175, 3, 132, 55, 96, 199, 47, 53, 205, 122, 31, 120, 121, 110,
            4, 85, 115, 4, 141, 134, 47, 191,
        ];
        let bs58 = bs58::encode(bytes).into_string();
        println!("private key in base58: {}", bs58);
    }

    #[test]
    fn from_json_to_keypair() {
        use serde_json::{from_reader, Result};
        use std::fs::File;

        let file = File::open("wallet.json").expect("File not found");

        // Deserialize the JSON content into the User struct.
        let keypair_bytes: Vec<u8> = from_reader(file).expect("Error while reading the file");

        let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
        println!(
            "You've loaded a Solana wallet: {}",
            keypair.try_pubkey().unwrap()
        );
    }

    #[test]
    fn airdop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::signature::{read_keypair_file, Keypair, Signer};

        const RPC_URL: &str = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 5_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;

        use solana_program::{pubkey::Pubkey, system_instruction::transfer};

        use solana_sdk::{
            signature::{read_keypair_file, Keypair, Signer},
            transaction::Transaction,
        };

        use std::str::FromStr;

        const RPC_URL: &str = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("FtD6Uk2wXEY6hqTDuP3paChLDDWKEA3659QmhUjvZQ1W").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_all() {
        use solana_sdk::{
            message::Message,
            signature::{read_keypair_file, Keypair, Signer},
            transaction::Transaction,
        };

        use solana_client::rpc_client::RpcClient;

        use solana_program::{pubkey::Pubkey, system_instruction::transfer};

        use std::str::FromStr;

        const RPC_URL: &str = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("FtD6Uk2wXEY6hqTDuP3paChLDDWKEA3659QmhUjvZQ1W").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees let fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // Deduct fee from lamports amount and create a TX with correct balance let transaction =
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn interact_pda() {
        use solana_sdk::{
            message::Message,
            signature::{read_keypair_file, Keypair, Signer},
            transaction::Transaction,
        };

        use solana_client::rpc_client::RpcClient;

        use solana_program::{pubkey::Pubkey, system_instruction::transfer};

        use std::str::FromStr;

        use crate::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs};

        const RPC_URL: &str = "https://api.devnet.solana.com";
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("./solana-dev-wallet.json").expect("Couldn't find wallet file");

        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"osozero".to_vec(),
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Now we can invoke the "complete" function let transaction =
        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here:
            https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
