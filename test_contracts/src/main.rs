use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::sync::Arc;
use ethers::types::U256;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // RPC URL for the Nitro Dev Node
    let rpc_url = "http://127.0.0.1:8547"; // Replace with your Nitro dev node URL if different
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Private Key (Replace with your private key, securely load it from ENV in production)
    let private_key = "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(1337u64); // Use the correct chain ID

    // Combine wallet and provider for signing
    let signer = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    // Recipient address
    let recipient: Address = "0xa6e41ffd769491a42a6e5ce453259b93983a22ef".parse()?;

    // Amount to send (5 ETH, converted to wei)
    let amount = U256::from(5u64) * U256::exp10(18); // 5 ETH in wei

    // Construct the transaction
    let tx = TransactionRequest::new()
        .to(recipient)
        .value(amount)
        .from(wallet.address());

    // Send the transaction
    let pending_tx = signer.send_transaction(tx, None).await?;

    println!("Transaction sent. Hash: {:?}", pending_tx.tx_hash());

    // Wait for the transaction to be mined
    let receipt = pending_tx.await?;
    println!("Transaction mined. Receipt: {:?}", receipt);

    Ok(())
}
