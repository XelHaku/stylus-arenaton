
use crate::call_contract::{call_contract_method, call_contract_method_signed};
use ethers::prelude::*;
use eyre::Result;
use serde::de::value;
use std::sync::Arc;

/// Function to get the name of the contract
pub async fn name(rpc_url: &str, contract_address: &str) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "name",
            "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
            "stateMutability": "pure",
            "type": "function"
        }
    ]"#;

    let contract_name: String = call_contract_method(
        "name",
        (), // No arguments
        abi_json,
        contract_address,
        rpc_url,
    )
    .await?;

    println!("\nContract Name: {}", contract_name);
    Ok(())
}

/// Function to get the total supply of the contract
pub async fn total_supply(rpc_url: &str, contract_address: &str) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "totalSupply",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;

    let total_supply: u128 = call_contract_method(
        "totalSupply",
        (), // No arguments
        abi_json,
        contract_address,
        rpc_url,
    )
    .await?;

    println!("\nTotal Supply: {}", total_supply);
    Ok(())
}

/// Function to get the balance of a specific address
pub async fn balance_of(
    owner_address: &str,
    rpc_url: &str,
    contract_address: &str,
) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "address", "name": "owner", "type": "address" }
            ],
            "name": "balanceOf",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;

    let owner: Address = owner_address.parse()?;

    let balance: U256 = call_contract_method(
        "balanceOf",
        owner, // Pass owner as argument
        abi_json,
        contract_address,
        rpc_url,
    )
    .await?;

    println!("\nBalance of {}: {}", owner, balance);
    Ok(())
}


pub async fn debug_mint_aton(
    contract_address: &str,
    private_key: &str,
    rpc_url: &str,
    chain_id: u64,
) -> Result<()> {
    let abi_json = r#"[{"inputs":[],"name":"debugMintAton","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#;

    // Create signer from private key
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "debugMintAton",
        (),
        abi_json,
        contract_address,
        signer,
        U256::from(100000),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}

pub async fn approve(
    contract_address: &str,
    private_key: &str,
    rpc_url: &str,
    chain_id: u64,
    spender: Address,
    value: U256,
) -> Result<()> {
    let abi_json = r#"[
  {
    "inputs": [
      { "internalType": "address", "name": "spender", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" }
    ],
    "name": "approve",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "nonpayable",
    "type": "function"
  }
]"#;

    // Create signer from private key
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

let receipt = call_contract_method_signed( // Remove <bool>
    "approve",
    (spender, value),
    abi_json,
    contract_address,
    signer,
    U256::from(0),
)
.await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}   



pub async fn stake_eth(
    contract_address: &str,
    private_key: &str,
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
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "payable",
    "type": "function"
  }
]"#;

    // Create signer from private key
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "stakeEth",
        player,
        abi_json,
        contract_address,
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


pub async fn initialize_contract(
    contract_address: &str,
    private_key: &str,
    rpc_url: &str,
    chain_id: u64,
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

    // Create signer from private key
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "initializeContract",
        (), // No arguments
        abi_json,
        contract_address,
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