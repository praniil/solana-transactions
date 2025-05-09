//to send SOL, you will nedd to interact with the System Program (User account for sending and receiveing SOL. Basically Wallet)
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://127.0.0.1:8899"),
        CommitmentConfig::confirmed(),
    );

    let from_keypair = Keypair::new();
    let to_keypair = Keypair::new();

    let transfer_ix = transfer(&from_keypair.pubkey(), &to_keypair.pubkey(), LAMPORTS_PER_SOL);

    let trasaction_signature = client.request_airdrop(&from_keypair.pubkey(), 5 * LAMPORTS_PER_SOL).await?;
    loop {
        if client.confirm_transaction(&trasaction_signature).await? {
            break;
        }
    }

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&from_keypair.pubkey()));
    transaction.sign(&[&from_keypair], client.get_latest_blockhash().await?);
    println!("public_key from: {}", from_keypair.pubkey());
    println!("public_key to: {}", to_keypair.pubkey());

    match client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    let confirmation = client.confirm_transaction(&trasaction_signature).await?;
    if confirmation {
        println!("Transaction confirmed");
    } else {
        println!("Transaction not confirmed");
    }

    let final_balance = client.get_balance(&to_keypair.pubkey()).await?;
    let final_balance_sender = client.get_balance(&from_keypair.pubkey()).await?;
    println!(
        "Final balance of recipient: {} SOL",
        final_balance as f64 / LAMPORTS_PER_SOL as f64
    );

    println!(
        "Final balance of sender: {} SOL",
        final_balance_sender as f64 / LAMPORTS_PER_SOL as f64
    );

    Ok(())
}