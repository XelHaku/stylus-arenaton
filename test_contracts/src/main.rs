// src/main.rs
mod call_contract;
mod methods;
mod constants;
mod players; 
mod erc20aton;
// Importa las funciones necesarias
use crate::players::fund_players_eth::fund_players_eth;
use crate::players::eth_balance::eth_balance;
use methods::{
    debug_mint_aton, stake_eth,
};
use erc20aton::{owner,name,total_supply,initialize_contract,grant_arenaton_role,approve,balance_of};
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
    // Obtiene el nombre del contrato del motor y del token ATON
    name(&env.rpc_url, &env.erc20aton_address).await?;
    // Obtiene el suministro total de tokens ATON
    total_supply(&env.rpc_url, &env.erc20aton_address).await?;
    let _owner = &WALLETS[0];
    let arenaton_engine_mock = &WALLETS[1];

    fund_players_eth("1000000000000000000", &env.rpc_url, env.chain_id, Some(2)).await?;
    owner(&env.rpc_url, &env.erc20aton_address).await?;
    // Inicializa el contrato (opcional)
     initialize_contract(&env.erc20aton_address, &_owner.private_key, &env.rpc_url, env.chain_id).await?;

    owner(&env.rpc_url, &env.erc20aton_address).await?;



    grant_arenaton_role(&env.erc20aton_address, &arenaton_engine_mock.address, &_owner.private_key, &env.rpc_url, env.chain_id).await?;


    eth_balance(&arenaton_engine_mock.address, &env.rpc_url).await?;

// mint_aton_from_eth(&env.erc20aton_address, U256::from(100), &arenaton_engine_mock.private_key, &env.rpc_url, env.chain_id).await?;


    // eth_balance(&arenaton_engine_mock.address, &env.rpc_url).await?;







































    // Obtiene el balance de ETH 
eth_balance(&env.erc20aton_address, &env.rpc_url).await?;


    // Obtiene el balance de tokens ATON del primer jugador después de la apuesta
    balance_of(_owner.address,  &env.rpc_url,  &env.erc20aton_address.as_str()).await?;
    balance_of(arenaton_engine_mock.address,  &env.rpc_url,  &env.erc20aton_address.as_str()).await?;


    // Imprime las direcciones de las carteras (opcional)
    print_wallets(Some(1));

    Ok(())
}
