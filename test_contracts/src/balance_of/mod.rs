use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use eyre::{Result, WrapErr};
use std::sync::Arc;

// Mark this function as async without the #[tokio::main] attribute
pub async fn balance_of() -> Result<()> {
    // RPC URL for your local Ethereum node
    let rpc_url = "http://127.0.0.1:8545";
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)
        .wrap_err("Failed to connect to the local Ethereum node")?);

    // Address to check (replace with the desired address)
    let target_address: Address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a".parse()
        .wrap_err("Invalid Ethereum address")?;

    // Fetch the balance
    let balance = provider
        .get_balance(target_address, None)
        .await
        .wrap_err("Failed to fetch balance from the local node")?;

    // Convert balance from Wei to Ether for readability
    let balance_in_ether = ethers::utils::format_units(balance, 18)
        .wrap_err("Failed to convert balance to Ether")?;

    // Print the balance
    println!("Balance of {:?}: {} ETH", target_address, balance_in_ether);

    Ok(())
}

#[tokio::main] // Only apply this to the main entry point
async fn main() {
    // This will only run when the module is called by itself
    if let Err(err) = balance_of().await {
        eprintln!("Error: {:?}", err);
    }
}



// // src/main.rs

// use ethers::prelude::*;
// use ethers::providers::{Http, Provider};
// use ethers::types::Address;
// use eyre::{Result, WrapErr};
// use std::sync::Arc;

// #[tokio::main]
// async fn main() -> Result<()> {
//     // RPC URL for your local Ethereum node
//     let rpc_url = "http://127.0.0.1:8547";
//     let provider = Arc::new(Provider::<Http>::try_from(rpc_url)
//         .wrap_err("Failed to connect to the local Ethereum node")?);

//     // Address to check (replace with the desired address)
//     let target_address: Address = "0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E".parse()
//         .wrap_err("Invalid Ethereum address")?;

//     // Fetch the balance
//     let balance = provider
//         .get_balance(target_address, None)
//         .await
//         .wrap_err("Failed to fetch balance from the local node")?;

//     // Convert balance from Wei to Ether for readability
//     let balance_in_ether = ethers::utils::format_units(balance, 18)
//         .wrap_err("Failed to convert balance to Ether")?;

//     // Print the balance
//     println!("Balance of {:?}: {} ETH", target_address, balance_in_ether);

//     Ok(())
// }