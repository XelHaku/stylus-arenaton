use ethers::prelude::*;
use ethers::abi::{Tokenizable, Detokenize};
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use ethers::contract::Contract;
use ethers::abi::Abi;
use eyre::{Result, WrapErr};
use std::sync::Arc;
use serde_json::from_str;

/// Generalized function to call a contract method (unsigned)
pub async fn call_contract_method<T: Tokenizable + Detokenize>(
    method_name: &str,
    args: impl Tokenizable,
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
pub async fn call_contract_method_signed<T: Tokenizable + Detokenize>(
    method_name: &str,
    args: impl Tokenizable,
    abi_json: &str,
    contract_address_str: &str,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<T> {
    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    let abi: Abi = from_str(abi_json).wrap_err("Error parsing ABI")?;
    let contract = Contract::new(contract_address, abi, signer);

    let tx = contract
        .method::<_, T>(method_name, args)?
        .send()
        .await?
        .await
        .wrap_err(format!("Failed to call '{}' method", method_name))?;

    Ok(tx)
}
