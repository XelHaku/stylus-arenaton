use ethers::abi::{Abi, Detokenize, Tokenize};
use ethers::prelude::*;
use eyre::{Result, WrapErr};
use std::sync::Arc;
use serde_json::from_str;

/// Generalized function to call a contract method (unsigned)
pub async fn call_contract_method<T: Detokenize>(
    method_name: &str,
    args: impl Tokenize,
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
pub async fn call_contract_method_signed(
    method_name: &str,
    args: impl Tokenize,
    abi_json: &str,
    contract_address_str: &str,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<Option<TransactionReceipt>> {
    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    let abi: Abi = serde_json::from_str(abi_json).wrap_err("Error parsing ABI")?;
    let contract = Contract::new(contract_address, abi, signer);

    // Send the transaction and await its receipt
    let tx = contract
        .method::<_, ()>(method_name, args)?
        .send()
        .await?
        .await
        .wrap_err(format!("Failed to call '{}' method", method_name))?;

    Ok(tx)
}
