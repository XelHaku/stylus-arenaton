// src/main.rs
mod call_contract;
mod methods;
mod constants;
mod players; // Add this line

use crate::players::fund_players_eth::fund_players_eth;
use crate::players::eth_balance::eth_balance;
use methods::{debug_mint_aton,approve, balance_of,total_supply,name,stake_eth,initialize_contract};
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use constants::wallets::{WALLETS,print_wallets};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let erc20aton_address = std::env::var("ERC20ATON_ADDRESS").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let engine_address = std::env::var("ENGINE_ADDRESS").unwrap_or_else(|_| "http://127.0.0.1:8545".into());

    let whale_private_key = std::env::var("PRIVATE_KEY").expect("Please set the PRIVATE_KEY environment variable");

    let chain_id = std::env::var("CHAIN_ID")
        .unwrap_or_else(|_| "412346".to_string())
        .parse::<u64>()?;
   
   
    initialize_contract(&erc20aton_address, whale_private_key, &rpc_url, chain_id).await?;
    name(&rpc_url, engine_address.as_str()).await?;
    name(&rpc_url, erc20aton_address.as_str()).await?;

    eth_balance(erc20aton_address.parse::<Address>()?, &rpc_url).await?;
    total_supply(&rpc_url, erc20aton_address.as_str()).await?;

    // Call the fund_players_eth function
    print_wallets(Some(1));

    fund_players_eth("1000000000000000000",&rpc_url, chain_id,Some(1)).await?;
balance_of(&WALLETS[0].address, &rpc_url, erc20aton_address.as_str()).await?;



stake_eth(&engine_address, &WALLETS[0].private_key, &rpc_url, chain_id, WALLETS[0].address.parse::<Address>()?, U256::from(1000000)).await?;
    eth_balance(erc20aton_address.parse::<Address>()?, &rpc_url).await?;

balance_of(&WALLETS[0].address, &rpc_url, erc20aton_address.as_str()).await?;





// debug_mint_aton(
//     &erc20aton_address, 
//     &WALLETS[0].private_key, 
//     &rpc_url, 
//     chain_id
// ).await?;

// balance_of(&WALLETS[0].address, &rpc_url, erc20aton_address.as_str()).await?;

// approve(&erc20aton_address, &WALLETS[0].private_key, &rpc_url, chain_id, WALLETS[2].address.parse::<Address>()?, U256::from(1000000)).await?;

    // Call the debugMintAton method
    // debug_mint_aton(erc20aton_address, signer).await?;

    Ok(())
}
