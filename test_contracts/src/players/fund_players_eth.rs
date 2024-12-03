// src/functions/fund_players_eth.rs

use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use crate::constants::wallets::{WALLETS, Wallet};

pub async fn fund_players_eth(
    amount: &str, // Amount in Wei
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

    // Parse the amount from string to U256
    let amount = U256::from_dec_str(amount)?;

    // Fund the specified amount to each player from the whale, up to the limit
    let mut funded_count = 0;
    for player_wallet in WALLETS {
        if limit.map_or(true, |l| funded_count < l) {
            let tx = whale_signer
                .send_transaction(TransactionRequest::pay(
                    player_wallet.address.parse::<Address>()?,
                    amount, // Use the parsed amount
                ), None)
                .await?
                .await?;

            match tx {
                Some(receipt) => {
                    println!("Funded player {} with {} Wei. Transaction hash: {:?}", 
                             player_wallet.address, amount, receipt.transaction_hash);
                }
                None => {
                    println!("Failed to fund player {}", player_wallet.address);
                }
            }

            funded_count += 1;
        } else {
            break; 
        }
    }

    Ok(())
}


