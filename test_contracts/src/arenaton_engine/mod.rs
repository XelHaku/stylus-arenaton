
use crate::call_contract::{call_contract_method, call_contract_method_signed};
use ethers::prelude::*;
use eyre::Result;
use serde::de::value;
use std::sync::Arc;
use crate::constants::env_vars::{get_env_vars, EnvVars};
use crate::constants::wallets::{WALLETS, Wallet};



pub async fn stake_eth(
    contract_address: &str,
    player_private_key: &str,
    rpc_url: &str,
    chain_id: u64,
    player: Address,
    value: U256,
) -> Result<()> {
    let abi_json = r#"[
{
        "inputs": [
            { "internalType": "address", "name": "_player", "type": "address" }
        ],
        "name": "stakeEth",
        "outputs": [
            { "internalType": "bool", "name": "", "type": "bool" }
        ],
        "stateMutability": "payable",
        "type": "function"
    }
]"#;


    let env = get_env_vars();

    // Create signer from private key
    let wallet = player_private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));
    let engine_address = env.engine_address;

    let receipt = call_contract_method_signed(
        "stakeEth",
        player,
        abi_json,
        &engine_address,
        signer,
        value,
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}

pub async fn add_event(
    event_id: &str,
    start_date: u64,
    sport: u8,
) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "string", "name": "event_id", "type": "string" },
                { "internalType": "uint64", "name": "start_date", "type": "uint64" },
                { "internalType": "uint8", "name": "sport", "type": "uint8" }
            ],
            "name": "addEvent",
            "outputs": [
                { "internalType": "bool", "name": "", "type": "bool" }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;

    // Convert event_id to String
    let event_id = event_id.to_string();
    let env = get_env_vars();

    let engine_address = env.engine_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

        let wallet = WALLETS[10].private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "addEvent",
        (event_id, start_date, sport),
        abi_json,
        &engine_address,
        signer,
        U256::zero(),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nEvent added successfully: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}

pub async fn grant_oracle_role(
    oracle_address: &str,
) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "address", "name": "account", "type": "address" }
            ],
            "name": "grantOracleRole",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;

    let env = get_env_vars();

    let engine_address = env.engine_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    // Create signer from private key
    let wallet = WALLETS[0].private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    // Convert oracle_address to Address type
    let oracle_address = oracle_address.parse::<Address>()?;

    let receipt = call_contract_method_signed(
        "grantOracleRole",
        oracle_address,
        abi_json,
        &engine_address,
        signer,
        U256::zero(),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nOracle role granted successfully: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}





pub async fn initialize_engine_contract(
) -> Result<()> {
    let abi_json = r#"[
  {
    "inputs": [],
    "name": "initializeContract",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "nonpayable", 
    "type": "function"
  }
]"#;
    let env = get_env_vars();

    let engine_address = env.engine_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;
    // Create signer from private key
    let wallet = WALLETS[0].private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "initializeContract",
        (), // No arguments
        abi_json,
        &engine_address,
        signer,
        U256::zero(), // No value sent
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}
