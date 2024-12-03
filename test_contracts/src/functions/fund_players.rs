// src/functions/fund_players.rs

use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use crate::constants::wallets::{WALLETS, Wallet};

pub async fn fund_players(
    rpc_url: &str,
    whale_private_key: &str,
    chain_id: u64,
) -> Result<()> {
    // Set up the whale signer
    let whale_wallet = whale_private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let whale_signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        whale_wallet,
    ));

    // Fund 1 ETH to each player from the whale
    for player_wallet in WALLETS {
        let tx = whale_signer
            .send_transaction(TransactionRequest::pay(
                player_wallet.address.parse::<Address>()?,
                U256::from(10u64.pow(18)), // 1 ETH
            ), None) // Add the missing BlockId argument (None in this case)
            .await?
            .await?;

        // Handle the Option<TransactionReceipt>
        match tx {
            Some(receipt) => {
                println!("Funded player {} with 1 ETH. Transaction hash: {:?}", 
                         player_wallet.address, receipt.transaction_hash);
            }
            None => {
                println!("Failed to fund player {}", player_wallet.address);
            }
        }
    }

    Ok(())
}