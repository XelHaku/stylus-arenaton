use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::contract::Contract;
use dotenv::dotenv;
use std::sync::Arc;
use ethers::abi::Abi;

// Add this line to import the necessary eyre types
use eyre::{Result, WrapErr};

pub async fn balance_of() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // RPC URL (Replace with your Ethereum node URL)
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8547".into());
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Contract address (Replace with your contract address)
    let contract_address: Address = "0x525c2aba45f66987217323e8a05ea400c65d06dc".parse()?;

    // Contract ABI (Replace with the actual ABI)
    let abi: Abi = serde_json::from_str(
        r#"[
   {
    "constant": true,
    "inputs": [
        {
            "name": "owner",
            "type": "address"
        }
    ],
    "name": "balanceOf",
    "outputs": [
        {
            "name": "",
            "type": "uint256"
        }
    ],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
}

]"#
    )
    .wrap_err("Error parsing ABI")?;

    // Initialize the contract instance
    let contract = Contract::new(contract_address, abi, Arc::new(provider));

    // Replace with the address you want to query
    let owner_address: Address = "0x525c2aba45f66987217323e8a05ea400c65d06dc".parse()?;

    // Call the "balanceOf" function on the smart contract
    let balance: U256 = contract
        .method::<_, U256>("balanceOf", owner_address)?
        .call()
        .await?;

    println!("Balance of {:?}: {}", owner_address, balance);

    Ok(())
}

#[tokio::main]
async fn main() {
    // This will only run when the module is called by itself
    if let Err(err) = balance_of().await {
        eprintln!("Error: {:?}", err);
    }
}
