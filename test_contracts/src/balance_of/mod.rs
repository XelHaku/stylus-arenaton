use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use ethers::contract::Contract;
use ethers::abi::Abi;
use dotenv::dotenv;
use std::sync::Arc;

// Import necessary eyre types
use eyre::{Result, WrapErr};

// Import tracing for logging
use tracing::{info, debug, error};
use tracing_subscriber;

pub async fn balance_of(owner_address_str: &str) -> Result<()> {
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
            "inputs": [
                { "internalType": "address", "name": "owner", "type": "address" }
            ],
            "name": "balanceOf",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;

    let abi: Abi = serde_json::from_str(abi_str)
        .wrap_err("Error parsing ABI")?;
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
