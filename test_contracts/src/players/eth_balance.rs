// src/functions/fund_players_eth.rs

use ethers::prelude::*;
use eyre::Result;




pub async fn eth_balance(
    address: Address,
    rpc_url: &str,
) -> Result<U256> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let balance = provider.get_balance(address, None).await?;
     println!("\nETH balance: {:?}", balance);
    Ok(balance)
}
