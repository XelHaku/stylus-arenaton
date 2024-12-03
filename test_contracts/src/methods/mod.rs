use crate::call_contract::{call_contract_method, call_contract_method_signed};
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

/// Function to get the name of the contract
pub async fn name(
    rpc_url: &str,
    contract_address: &str,
) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "name",
            "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
            "stateMutability": "pure",
            "type": "function"
        }
    ]"#;

    // Use call_contract_method to call the `name` function
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

    // Parse the owner address
    let owner: Address = owner_address.parse()?;

    // Use call_contract_method to call the `balanceOf` function
    let balance: U256 = call_contract_method(
        "balanceOf",
        owner,
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

    // Use call_contract_method_signed to call the `debugMintAton` function
    call_contract_method_signed::<()>(
        "debugMintAton",
        (), // No arguments
        abi_json,
        contract_address,
        signer,
    )
    .await?;

    println!("debugMintAton executed successfully");
    Ok(())
}
