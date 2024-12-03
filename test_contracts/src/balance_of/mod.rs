use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
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

pub async fn balance_of(owner_address_str: &str) -> Result<()> {
    // Load environment variables
    dotenv().ok();
    info!("Environment variables loaded");

    // Get RPC URL from environment or fallback to default
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8547".into());
    info!("Using RPC URL: {}", rpc_url);

    let provider = Provider::<Http>::try_from(rpc_url)
        .wrap_err("Failed to create provider")?;
    info!("Provider initialized");

    // Get contract address from environment or fallback to default
    let contract_address_str = std::env::var("CONTRACT_ADDRESS")
        .unwrap_or_else(|_| "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a".into());
    info!("Using contract address: {}", contract_address_str);

    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    info!("Contract address parsed: {:?}", contract_address);

    // Load ABI dynamically from file
    let abi_path = Path::new("src/abi/IATON.json"); // Adjust path to match your project structure
    let abi_json = fs::read_to_string(abi_path).wrap_err("Failed to read ABI file")?;
    info!("ABI file loaded from: {:?}", abi_path);

    let abi: Abi = serde_json::from_str(&abi_json)
        .wrap_err("Error parsing ABI from file")?;
    info!("Contract ABI parsed");

    // Initialize the contract instance
    let contract = Contract::new(contract_address, abi, Arc::new(provider));
    info!("Contract instance created");

    // Parse the owner address passed as argument
    let owner_address: Address = owner_address_str
        .parse()
        .wrap_err("Invalid owner address")?;
    info!("Using owner address: {:?}", owner_address);

    // Call the "balanceOf" function on the smart contract
    info!("Calling 'balanceOf' function on the contract");
    let balance: U256 = contract
        .method::<_, U256>("balanceOf", owner_address)?
        .call()
        .await
        .wrap_err("Failed to call 'balanceOf' function")?;
    info!("Contract 'balanceOf' function called successfully");

    println!("Balance of {}: {}", owner_address, balance);

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Example owner address (replace with an actual address to query)
    let address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";

    // Run the `balance_of` function and handle any errors
    if let Err(err) = balance_of(address).await {
        error!("Error: {:?}", err);
    }
}
