
use crate::call_contract::{call_contract_method, call_contract_method_signed};
use ethers::prelude::*;
use eyre::Result;
use serde::de::value;
use std::sync::Arc;
use crate::constants::env_vars::{get_env_vars, EnvVars};

use crate::constants::wallets::{WALLETS, Wallet};


/// Function to get the owner of the contract
pub async fn owner() -> Result<()> {
    let abi_json = r#"
    [
        {
            "inputs": [],
            "name": "owner",
            "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]
    "#;
    let env = get_env_vars();

    // Debug: Print the ABI to ensure it's correct
    println!("ABI: {}", abi_json);

    let contract_owner: Address = call_contract_method(
        "owner",
        (),
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    ).await?;

    println!("\nContract owner: {}", contract_owner);
    Ok(())
}


/// Function to get the name of the contract
pub async fn name() -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "name",
            "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
            "stateMutability": "pure",
            "type": "function"
        }
    ]"#;
    let env = get_env_vars();

    let contract_name: String = call_contract_method(
        "name",
        (), // No arguments
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    )
    .await?;

    println!("\nContract Name: {}", contract_name);
    Ok(())
}

/// Function to get the total supply of the contract
pub async fn total_supply() -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "totalSupply",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;
    let env = get_env_vars();

    let total_supply: u128 = call_contract_method(
        "totalSupply",
        (), // No arguments
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    )
    .await?;

    println!("\nTotal Supply: {}", total_supply);
    Ok(())
}

/// Function to get the balance of a specific address
pub async fn balance_of(
    owner_address: &str
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
    let env = get_env_vars();

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let owner: Address = owner_address.parse()?;

    let balance: U256 = call_contract_method(
        "balanceOf",
        owner, // Pass owner as argument
        abi_json,
        &erc20aton_address,
        &rpc_url,
    )
    .await?;

    println!("\nATON balance of {}: {}", owner, balance);
    Ok(())
}


pub async fn approve(
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
    let env = get_env_vars();

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    // Create signer from private key
    let wallet = WALLETS[0].private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

let receipt = call_contract_method_signed( // Remove <bool>
    "approve",
    (spender, value),
    abi_json,
    &erc20aton_address,
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





pub async fn initialize_erc20aton_contract(
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

    let erc20aton_address = env.erc20aton_address;
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
        &erc20aton_address,
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


pub async fn grant_arenaton_role(
    arenaton_engine_address: &str,
) -> Result<()> {
let abi_json = r#"[
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "account",
                "type": "address"
            }
        ],
        "name": "grantArenatonRole",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    }
]"#;


    
    let env = get_env_vars();

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;
    // Parse contract addresses
    let arenaton_engine_addr = arenaton_engine_address.parse::<Address>()?;

    // Create signer from private key
    let wallet = WALLETS[0].private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    // Call the contract method, passing arenaton_engine_addr as the function argument
    let receipt = call_contract_method_signed(
        "grantArenatonRole",
        arenaton_engine_addr,  // the argument to 'grantArenatonRole'
        abi_json,
        &erc20aton_address,
        signer,
        U256::zero(),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    

    Ok(())
}




// pub async fn mint_aton( erc20aton_address: &str,value: U256,    private_key: &str,
//     ,
//     chain_id: u64) -> Result<()> {

// /// Function to get the owner of the contract
//     let abi_json = r#"
//     [
//         {
//             "inputs": [],
//             "name": "mintAtonFromEth",
//             "outputs": [{ "internalType": "address", "name": "", "type": "bool" }],
//             "stateMutability": "payable",
//             "type": "function"
//         }
//     ]
//     "#;
//     // function mintAtonFromEth() external payable returns (bool);

//     // Parse contract addresses
//     // let arenaton_engine_addr = erc20aton_address.parse::<Address>()?;

//     // Create signer from private key
//     let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
//     let provider = Provider::<Http>::try_from(rpc_url)?;
//     let signer = Arc::new(SignerMiddleware::new(provider, wallet));

//     // Call the contract method, passing arenaton_engine_addr as the function argument
//     let receipt = call_contract_method_signed(
//         "mintAtonFromEth",
//         (),  // the argument to 'grantArenatonRole'
//         abi_json,
//         erc20aton_address,
//         signer,
//         value,
//     )
//     .await?;

//     match receipt {
//         Some(receipt) => println!("\nTransaction successful: {:?}", receipt),
//         None => println!("\nTransaction executed successfully, but no receipt was returned."),
//     }

//     Ok(())
// }
