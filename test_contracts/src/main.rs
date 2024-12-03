// src/main.rs
mod call_contract;
mod methods;
mod constants;
mod functions; // Add this line

use crate::functions::fund_players::fund_players;
use methods::debug_mint_aton;
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use constants::wallets::{print_wallets};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let contract_address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";
    let whale_private_key = std::env::var("PRIVATE_KEY").expect("Please set the PRIVATE_KEY environment variable");

    let chain_id = std::env::var("CHAIN_ID")
        .unwrap_or_else(|_| "412346".to_string())
        .parse::<u64>()?;

    // Call the fund_players function
    fund_players(&rpc_url, &whale_private_key, chain_id).await?;


    print_wallets();

    // Call the debugMintAton method
    // debug_mint_aton(contract_address, signer).await?;

    Ok(())
}
