// src/main.rs
mod call_contract;
mod methods;
mod constants;
mod players; 

// Importa las funciones necesarias
use crate::players::fund_players_eth::fund_players_eth;
use crate::players::eth_balance::eth_balance;
use methods::{
    debug_mint_aton, approve, balance_of, total_supply, name, stake_eth, initialize_contract,
};
use ethers::prelude::*;
use eyre::Result;


use constants::wallets::{WALLETS, print_wallets};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Obtiene las variables de entorno
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let erc20aton_address = std::env::var("ERC20ATON_ADDRESS")
        .unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let engine_address = std::env::var("ENGINE_ADDRESS")
        .unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let whale_private_key = std::env::var("PRIVATE_KEY")
        .expect("Por favor, configura la variable de entorno PRIVATE_KEY");
    let chain_id = std::env::var("CHAIN_ID")
        .unwrap_or_else(|_| "412346".to_string())
        .parse::<u64>()?;

    // Inicializa el contrato (opcional)
    // initialize_contract(&erc20aton_address, &whale_private_key, &rpc_url, chain_id).await?;

    // Obtiene el nombre del contrato del motor y del token ATON
    name(&rpc_url, engine_address.as_str()).await?;
    name(&rpc_url, erc20aton_address.as_str()).await?;

    // Obtiene el balance de ETH del contrato del token ATON
    eth_balance(erc20aton_address.parse::<Address>()?, &rpc_url).await?;

    // Obtiene el suministro total de tokens ATON
    total_supply(&rpc_url, erc20aton_address.as_str()).await?;

    // Imprime las direcciones de las carteras (opcional)
    print_wallets(Some(1));

    // Envía ETH a los jugadores
    fund_players_eth("1000000000000000000", &rpc_url, chain_id, Some(1)).await?;

    // Obtiene el balance de tokens ATON del primer jugador
    balance_of(&WALLETS[0].address, &rpc_url, erc20aton_address.as_str()).await?;

    // Realiza una apuesta de ETH en el motor
    stake_eth(
        &engine_address,
        &WALLETS[0].private_key,
        &rpc_url,
        chain_id,
        WALLETS[0].address.parse::<Address>()?,
        U256::from(1000000),
    )
    .await?;

    // Obtiene el balance de ETH del contrato del token ATON después de la apuesta
    eth_balance(erc20aton_address.parse::<Address>()?, &rpc_url).await?;

    // Obtiene el balance de tokens ATON del primer jugador después de la apuesta
    balance_of(&WALLETS[0].address, &rpc_url, erc20aton_address.as_str()).await?;

    // ... (resto del código)

    Ok(())
}