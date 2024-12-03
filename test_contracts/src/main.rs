// src/main.rs

// Import the eyre crate for error handling
use eyre::Result;

// Declare the modules
mod debug_mint_aton;
mod name;
mod balance_of;

// Import the functions from the modules
use debug_mint_aton::debug_mint_aton;
use name::name;
use balance_of::balance_of;
#[tokio::main]
async fn main() -> Result<()> {
    // Call the functions and propagate errors using `?`
    // debug_mint_aton().await?;
    name().await?;


    let player1 = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";
    balance_of(player1).await?;


    // addEvent(abc,1750000000).await?
// stake(ATON,player2,team1).await?
   
    // swap
  
    Ok(())
}
