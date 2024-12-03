mod call_contract;
mod methods;

use methods::{balance_of, debug_mint_aton, name};
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Define RPC URL and contract address
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".into());
    let contract_address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";
    let player1 = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";

    // Call the name function
    name(&rpc_url, contract_address).await?;

    // Get balance for player address
    balance_of(player1, &rpc_url, contract_address).await?;

    // Mint debug ATON tokens (requires signing)
    let private_key = std::env::var("PRIVATE_KEY").expect("Please set the PRIVATE_KEY environment variable");
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(1337u64);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(&rpc_url)?,
        wallet,
    ));
    debug_mint_aton(contract_address, signer).await?;

    Ok(())
}
