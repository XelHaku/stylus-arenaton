
use ethers::prelude::*;
use eyre::Result;
 use eyre::WrapErr;



pub async fn eth_balance(
    address: &str,
    rpc_url: &str,
) -> Result<U256> {
    let provider = Provider::<Http>::try_from(rpc_url)?;

        let _address: Address = address
        .parse()
        .wrap_err("Invalid contract address")?;
    let balance = provider.get_balance(_address, None).await?;
     println!("\nETH balance of {:?}: {:?}", address, balance);
    Ok(balance)
}
