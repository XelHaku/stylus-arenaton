use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use dotenv::dotenv;
use std::sync::Arc;

abigen!(
    IATON,
    r#"
    [
        function debugMintAton() external
        function transfer(address to, uint256 value) external returns (bool)
        function donateAton() external
        function stakeEth(address _player) external
        function stakeAton(address _player, uint256 _amount) external
        function swap(uint256 amount) external
        error InsufficientBalance(address, uint256, uint256)
        error InsufficientAllowance(address, address, uint256, uint256)
        error ZeroEther(address)
    ]
    "#
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from a .env file (if present)
    dotenv().ok();

    // Replace with your actual RPC URL
    let rpc_url = "http://127.0.0.1:8547";

    // Load your private key securely from an environment variable
    let private_key = std::env::var("PRIVATE_KEY")
        .expect("Please set the PRIVATE_KEY environment variable");

    // Create a provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Create a wallet from the private key
    let wallet: LocalWallet = private_key.parse()?;

    // Connect the wallet to the provider
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Contract address
    let contract_address = "0xa6e41ffd769491a42a6e5ce453259b93983a22ef"
        .parse::<Address>()?;

    // Instantiate the contract
    let contract = IATON::new(contract_address, client.clone());

    // Set the amount of ETH to send with the transaction (10^-15 ETH)
    let value = ethers::utils::parse_ether("0.000000000000001")?;

    // Send the transaction calling debugMintAton with value
    let tx = contract
        .debug_mint_aton()
        .value(value) // Set the value to send with the transaction
        .send()
        .await?;

    // Wait for the transaction to be mined
    let receipt = tx.await?;

    // Check if the transaction was successful
    if let Some(receipt) = receipt {
        println!("Transaction succeeded in block: {:?}", receipt.block_number);
    } else {
        println!("Transaction pending or failed");
    }

    Ok(())
}
