// src/functions/fund_players_eth.rs

use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use crate::constants::wallets::{WALLETS, Wallet};

pub async fn fund_players_eth(
    rpc_url: &str,
    chain_id: u64,
    limit: Option<u64>,
) -> Result<()> {
    let whale_private_key = std::env::var("PRIVATE_KEY").expect("Please set the PRIVATE_KEY environment variable");
    // Set up the whale signer
    let whale_wallet = whale_private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let whale_signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        whale_wallet,
    ));

    // Fund 1 ETH to each player from the whale, up to the limit
    let mut funded_count = 0;
    for player_wallet in WALLETS {
        if limit.map_or(true, |l| funded_count < l) {
            let tx = whale_signer
                .send_transaction(TransactionRequest::pay(
                    player_wallet.address.parse::<Address>()?,
                    U256::from(10u64.pow(18)), // 1 ETH
                ), None)
                .await?
                .await?;

            match tx {
                Some(receipt) => {
                    println!("Funded player {} with 1 ETH. Transaction hash: {:?}", 
                             player_wallet.address, receipt.transaction_hash);
                }
                None => {
                    println!("Failed to fund player {}", player_wallet.address);
                }
            }

            funded_count += 1;
        } else {
            break; // Stop funding if the limit is reached
        }
    }

    Ok(())
}