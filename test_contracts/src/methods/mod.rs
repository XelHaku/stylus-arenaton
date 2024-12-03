
use crate::call_contract::{call_contract_method, call_contract_method_signed};
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

/// Function to get the name of the contract
pub async fn name(rpc_url: &str, contract_address: &str) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "name",
            "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
            "stateMutability": "pure",
            "type": "function"
        }
    ]"#;

    let contract_name: String = call_contract_method(
        "name",
        (), // No arguments
        abi_json,
        contract_address,
        rpc_url,
    )
    .await?;

    println!("Contract Name: {}", contract_name);
    Ok(())
}

/// Function to get the total supply of the contract
pub async fn total_supply(rpc_url: &str, contract_address: &str) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "totalSupply",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;

    let total_supply: u128 = call_contract_method(
        "totalSupply",
        (), // No arguments
        abi_json,
        contract_address,
        rpc_url,
    )
    .await?;

    println!("Total Supply: {}", total_supply);
    Ok(())
}

/// Function to get the balance of a specific address
pub async fn balance_of(
    owner_address: &str,
    rpc_url: &str,
    contract_address: &str,
) -> Result<()> {
    let abi_json = r#"[
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

    let owner: Address = owner_address.parse()?;

    let balance: U256 = call_contract_method(
        "balanceOf",
        owner, // Pass owner as argument
        abi_json,
        contract_address,
        rpc_url,
    )
    .await?;

    println!("Balance of {}: {}", owner, balance);
    Ok(())
}

/// Function to execute `debugMintAton`
pub async fn debug_mint_aton(
    contract_address: &str,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<()> {
    let abi_json = r#"[{"inputs":[],"name":"debugMintAton","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#;

    let receipt = call_contract_method_signed(
        "debugMintAton",
        (), // No arguments
        abi_json,
        contract_address,
        signer,
    )
    .await?;

    match receipt {
        Some(receipt) => println!("Transaction successful: {:?}", receipt),
        None => println!("Transaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}
