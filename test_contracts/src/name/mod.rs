use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::contract::Contract;
use ethers::abi::Abi;
use dotenv::dotenv;
use std::sync::Arc;

// Import necessary eyre types
use eyre::{Result, WrapErr};

pub async fn name() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // RPC URL (Replace with your Ethereum node URL)
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8547".into());
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Contract address (Replace with your contract address)
    let contract_address: Address = std::env::var("CONTRACT_ADDRESS")
        .unwrap_or_else(|_| "0x525c2aba45f66987217323e8a05ea400c65d06dc".into())
        .parse()
        .wrap_err("Invalid contract address")?;

    // Contract ABI (Replace with the actual ABI)
    let abi: Abi = serde_json::from_str(
        r#"[
            {
                "inputs": [],
                "name": "name",
                "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
                "stateMutability": "pure",
                "type": "function"
            }
        ]"#,
    )
    .wrap_err("Error parsing ABI")?; // Added the `?` operator here

    // Initialize the contract instance
    let contract = Contract::new(contract_address, abi, Arc::new(provider));

    // Call the "name" function on the smart contract
    let contract_name: String = contract
        .method::<_, String>("name", ())?
        .call()
        .await?;

    println!("Contract Name: {}", contract_name);

    Ok(())
}

#[tokio::main]
async fn main() {
    // Run the `name` function and handle any errors
    if let Err(err) = name().await {
        eprintln!("Error: {:?}", err);
    }
}
