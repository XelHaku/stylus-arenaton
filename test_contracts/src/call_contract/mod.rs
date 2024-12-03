use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::{Address, U256};
use ethers::contract::Contract;
use ethers::abi::Abi;
use dotenv::dotenv;
use std::sync::Arc;
use eyre::{Result, WrapErr};
use tracing::{info, error};
use tracing_subscriber;
use serde_json::from_str;

/// Generalized function to call a contract method (unsigned)
async fn call_contract_method<T: serde::de::DeserializeOwned>(
    method_name: &str,
    args: impl ethers::abi::Tokenizable,
    abi_json: &str,
    contract_address_str: &str,
    provider_url: &str,
) -> Result<T> {
    let provider = Provider::<Http>::try_from(provider_url).wrap_err("Failed to create provider")?;
    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    let abi: Abi = from_str(abi_json).wrap_err("Error parsing ABI")?;
    let contract = Contract::new(contract_address, abi, Arc::new(provider));

    let result: T = contract
        .method::<_, T>(method_name, args)?
        .call()
        .await
        .wrap_err(format!("Failed to call '{}' method", method_name))?;

    Ok(result)
}

/// Generalized function to call a contract method with signing
async fn call_contract_method_signed<T: serde::de::DeserializeOwned>(
    method_name: &str,
    args: impl ethers::abi::Tokenizable,
    abi_json: &str,
    contract_address_str: &str,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<T> {
    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    let abi: Abi = from_str(abi_json).wrap_err("Error parsing ABI")?;
    let contract = Contract::new(contract_address, abi, signer);

    let result: T = contract
        .method::<_, T>(method_name, args)?
        .send()
        .await?
        .await
        .wrap_err(format!("Failed to call '{}' method", method_name))?;

    Ok(result)
}

/// Function to get the name of the contract
pub async fn get_contract_name(
    provider_url: &str,
    contract_address_str: &str,
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

    let name: String = call_contract_method("name", (), abi_json, contract_address_str, provider_url).await?;
    println!("Contract Name: {}", name);

    Ok(())
}

/// Function to get the balance of a specific address
pub async fn get_balance_of(
    owner_address: &str,
    provider_url: &str,
    contract_address_str: &str,
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

    let owner: Address = owner_address.parse().wrap_err("Invalid owner address")?;
    let balance: U256 = call_contract_method("balanceOf", owner, abi_json, contract_address_str, provider_url).await?;
    println!("Balance of {}: {}", owner, balance);

    Ok(())
}

/// Function to execute `debugMintAton`
pub async fn debug_mint_aton(
    contract_address_str: &str,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<()> {
    let abi_json = r#"[{"inputs":[],"name":"debugMintAton","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#;

    call_contract_method_signed::<()>("debugMintAton", (), abi_json, contract_address_str, signer).await?;
    println!("debugMintAton executed successfully");

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let provider_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let contract_address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";
    let owner_address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";

    if let Err(err) = get_contract_name(&provider_url, contract_address).await {
        error!("Error getting contract name: {:?}", err);
    }

    if let Err(err) = get_balance_of(owner_address, &provider_url, contract_address).await {
        error!("Error getting balance: {:?}", err);
    }

    let private_key = std::env::var("PRIVATE_KEY").expect("Please set the PRIVATE_KEY environment variable");
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(1337u64);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(&provider_url).unwrap(),
        wallet,
    ));

    if let Err(err) = debug_mint_aton(contract_address, signer).await {
        error!("Error executing debugMintAton: {:?}", err);
    }
}
