// swap_aton_to_eth/mod.rs
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::{Address, U256};
use ethers::contract::Contract;
use dotenv::dotenv;
use std::sync::Arc;
use ethers::abi::Abi;
use eyre::{Result, WrapErr};

pub async fn swap_aton_to_eth() -> Result<()> {
    dotenv().ok();

    // RPC URL (Replace with your Ethereum node URL)
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Private Key (Replace with your private key, securely load it from ENV in production)
    let private_key = std::env::var("PRIVATE_KEY_NODE")
        .expect("Please set the PRIVATE_KEY_NODE environment variable");
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(1337u64); // Use the correct chain ID 

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    // ATON Token address and DEX Router address (Replace with your actual addresses)
    let aton_address: Address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a".parse()?; // Replace with ATON token address
    let router_address: Address = "0xYourDexRouterAddressHere".parse()?; // Replace with DEX router address

    // Contract ABIs
    let erc20_abi: Abi = serde_json::from_str(r#"[{"constant":true,"inputs":[],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"type":"function"},{"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"approve","outputs":[],"type":"function"}]"#)
        .wrap_err("Error parsing ERC20 ABI")?;
    let router_abi: Abi = serde_json::from_str(r#"[{"inputs":[{"name":"amountIn","type":"uint256"},{"name":"amountOutMin","type":"uint256"},{"name":"path","type":"address[]"},{"name":"to","type":"address"},{"name":"deadline","type":"uint256"}],"name":"swapExactTokensForETH","outputs":[{"name":"","type":"uint256[]"}],"stateMutability":"nonpayable","type":"function"}]"#)
        .wrap_err("Error parsing Router ABI")?;

    // Initialize contracts
    let aton_contract = Contract::new(aton_address, erc20_abi, signer.clone());
    let router_contract = Contract::new(router_address, router_abi, signer.clone());

    // Get ATON balance
    let my_address = signer.address();
    let aton_balance: U256 = aton_contract
        .method("balanceOf", my_address)?
        .call()
        .await
        .wrap_err("Failed to fetch ATON balance")?;

    // Swap half of ATON balance
    let half_aton = aton_balance / 2;

    // Approve the DEX router to spend ATON
    let approve_tx = aton_contract
        .method::<_, H256>("approve", (router_address, half_aton))?
        .send()
        .await?;
    approve_tx.await?;

    println!("Approved {} ATON for swapping.", half_aton);

    // Define the swap path: ATON -> ETH
    let path = vec![aton_address, "0xYourWETHAddressHere".parse()?]; // Replace with the wrapped ETH (WETH) address

    // Swap on the DEX
    let deadline = U256::from((std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs()
        + 300) as u64); // 5 minutes from now

    // let swap_tx = router_contract
    //     .method::<_, Vec<U256>>(
    //         "swapExactTokensForETH",
    //         (
    //             half_aton,
    //             U256::from(1), // Minimum amount of ETH to accept (adjust this as needed)
    //             path,
    //             my_address,
    //             deadline,
    //         ),
    //     )?
    //     .send()
    //     .await?;

    println!("Swap transaction sent. Hash: {:?}", swap_tx.tx_hash());

    let receipt = swap_tx.await?;
    println!("Swap transaction receipt: {:?}", receipt);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = swap_aton_to_eth().await {
        eprintln!("Error: {:?}", err);
    }
}
