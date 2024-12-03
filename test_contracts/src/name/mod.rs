use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::contract::Contract;
use ethers::abi::Abi;
use dotenv::dotenv;
use std::sync::Arc;
use std::fs;
use std::path::Path;

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
        .unwrap_or_else(|_| "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a".into());
    info!("Using contract address: {}", contract_address_str);

    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    info!("Contract address parsed: {:?}", contract_address);

    // Load ABI from file
    let abi_path = Path::new("src/abi/IATON.json");
    let abi_json = fs::read_to_string(abi_path).wrap_err("Failed to read ABI file")?;
    info!("ABI file loaded from: {:?}", abi_path);

    let abi: Abi = serde_json::from_str(&abi_json)
        .wrap_err("Error parsing ABI from file")?;
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
