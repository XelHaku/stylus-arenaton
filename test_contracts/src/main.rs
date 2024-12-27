mod call_contract;
mod methods;
mod constants;
mod players;
mod erc20aton;
mod arenaton_engine;

use crate::players::fund_players_eth::fund_players_eth;
use crate::players::eth_balance::eth_balance;
use erc20aton::{owner, name, total_supply, initialize_erc20aton_contract, grant_arenaton_role, approve, balance_of};
use arenaton_engine::{add_event, grant_oracle_role, stake_eth,initialize_engine_contract};

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
    let _oracle = &WALLETS[10];
    let engine_address = env.engine_address;

    // Fund players with ETH
    fund_players_eth("10000000000000000", Some(2)).await?;

    // Get contract owner
    owner().await?;

    // Initialize the contract (optional)
    initialize_erc20aton_contract().await?;
    initialize_engine_contract().await?;

    // Grant Arenaton role
    grant_arenaton_role(&engine_address).await?;

    // Grant Oracle role
    grant_oracle_role( _oracle.address).await?;



    // Print wallet addresses (optional)
    print_wallets(Some(1));

    // Get the balance of ATON tokens for the owner
    balance_of(_owner.address).await?;

    // Get the balance of ATON tokens for the engine address
    balance_of(&engine_address).await?;

    Ok(())
}
