use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use ethers::contract::Contract;
use dotenv::dotenv;
use std::sync::Arc;
use ethers::abi::Abi;

// Add this line to import the necessary eyre types
use eyre::{Result, WrapErr}; 

#[tokio::main]
async fn main() -> Result<()> { // Use eyre::Result
    // Load environment variables
    dotenv().ok();

    // RPC URL (Replace with your Ethereum node URL)
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Private Key (Replace with your private key, securely load it from ENV in production)
    let private_key = std::env::var("PRIVATE_KEY_NODE")
        .expect("Please set the PRIVATE_KEY_NODE environment variable");
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(1337u64); // Use the correct chain ID (1337 in this case)

    // Combine wallet and provider
    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    // Contract address (Replace with your contract address)
    let contract_address: Address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a".parse()?; 

    // Contract ABI (Replace with the actual ABI)
    // Make sure this is the correct ABI for the "increment" function
    let abi: Abi = serde_json::from_str(r#"[{"inputs":[],"name":"increment","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#)
        .wrap_err("Error parsing ABI")?; 

    // Initialize the contract instance
    let contract = Contract::new(contract_address, abi, signer.clone());

    // Call the "increment" function on the smart contract
    let method = contract.method::<_, ()>("increment", ())?; 
    let tx = method
        .send()
        .await?; 

    println!("Transaction hash: {:?}", tx.tx_hash());

    // Wait for the transaction to be mined
    let receipt = tx.await?;
    println!("Transaction receipt: {:?}", receipt);

    Ok(())
}