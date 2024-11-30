use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::contract::Contract;
use ethers::abi::Abi;
use dotenv::dotenv;
use std::sync::Arc;

// Import necessary eyre types
use eyre::{Result, WrapErr};

// Import tracing for logging
use tracing::{info, debug, error};
use tracing_subscriber;

pub async fn name() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    info!("Environment variables loaded");

    // RPC URL (Replace with your Ethereum node URL)
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8547".into());
    info!("Using RPC URL: {}", rpc_url);

    let provider = Provider::<Http>::try_from(rpc_url)
        .wrap_err("Failed to create provider")?;
    info!("Provider initialized");

    // Contract address (Replace with your contract address)
    let contract_address_str = std::env::var("CONTRACT_ADDRESS")
        .unwrap_or_else(|_| "0x525c2aba45f66987217323e8a05ea400c65d06dc".into());
    info!("Using contract address: {}", contract_address_str);

    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    info!("Contract address parsed: {:?}", contract_address);

    // Contract ABI (Replace with the actual ABI)
    let abi_str = r#"[
            {
                "inputs": [],
                "name": "name",
                "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
                "stateMutability": "pure",
                "type": "function"
            }
        ]"#;

    let abi: Abi = serde_json::from_str(abi_str)
        .wrap_err("Error parsing ABI")?;
    info!("Contract ABI parsed");

    // Initialize the contract instance
    let contract = Contract::new(contract_address, abi, Arc::new(provider));
    info!("Contract instance created");

    // Call the "name" function on the smart contract
    info!("Calling 'name' function on the contract");
    let contract_name: String = contract
        .method::<_, String>("name", ())?
        .call()
        .await
        .wrap_err("Failed to call 'name' function")?;
    info!("Contract 'name' function called successfully");

    println!("Contract Name: {}", contract_name);

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Run the `name` function and handle any errors
    if let Err(err) = name().await {
        error!("Error: {:?}", err);
    }
}
