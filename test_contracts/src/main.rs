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
[package]
name = "test_contracts"
version = "0.1.0"
edition = "2021"

[dependencies]
dotenv = "0.15.0"
ethers = "2.0.14"
tokio = "1.41.1"
ompiling ethers-contract v2.0.14
   Compiling ethers-middleware v2.0.14
   Compiling ethers v2.0.14
   Compiling test_contracts v0.1.0 (/home/xel/git/stylus-hello-world/test_contracts)
error[E0433]: failed to resolve: could not find `main` in `tokio`
  --> src/main.rs:25:10
   |
25 | #[tokio::main]
   |          ^^^^ could not find `main` in `tokio`

error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:58:14
   |
58 |       let tx = contract
   |  ______________^
59 | |         .debug_mint_aton()
60 | |         .value(value) // Set the value to send with the transaction
   | |_____________________^ creates a temporary value which is freed while still in use
61 |           .send()
62 |           .await?;
   |                  - temporary value is freed at the end of this statement
...
65 |       let receipt = tx.await?;
   |                     -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0752]: `main` function is not allowed to be `async`
  --> src/main.rs:26:1
   |
26 | async fn main() -> Result<(), Box<dyn std::error::Error>> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`

Some errors have detailed explanations: E0433, E0716, E0752.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `test_contracts` (bin "test_contracts") due to 3 previous errors
xel@BlueBlack:~/git/stylus-hello-world/test_contracts$ 