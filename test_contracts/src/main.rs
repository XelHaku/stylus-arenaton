mod call_contract;
mod methods;
mod constants;
mod players;
mod erc20aton;

use crate::players::fund_players_eth::fund_players_eth;
use crate::players::eth_balance::eth_balance;
use methods::{debug_mint_aton, stake_eth};
use erc20aton::{owner, name, total_supply, initialize_contract, grant_arenaton_role, approve, balance_of};
use ethers::prelude::*;
use eyre::Result;
use constants::wallets::{WALLETS, print_wallets};
use constants::env_vars::{get_env_vars, EnvVars};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = get_env_vars();
    println!("RPC URL is: {}", env.rpc_url);

    // Get contract and token ATON name
    name().await?;
    
    // Get total token supply
    total_supply().await?;

    let _owner = &WALLETS[0];
    let engine_address = env.engine_address;

    // Fund players with ETH
    fund_players_eth("1000000000000000000", Some(2)).await?;

    // Get contract owner
    owner().await?;

    // Initialize the contract (optional)
    // initialize_contract().await?;

    // Grant Arenaton role
    grant_arenaton_role(&engine_address).await?;



    // Print wallet addresses (optional)
    print_wallets(Some(1));

    // Get the balance of ATON tokens for the owner
    balance_of(_owner.address).await?;

    // Get the balance of ATON tokens for the engine address
    balance_of(&engine_address).await?;

    Ok(())
}
