// src/main.rs
// mod debug_mint_aton;
// use debug_mint_aton::debug_mint_aton;

mod balance_of;
use balance_of::balance_of;

use eyre::Result; // Import Result from eyre

#[tokio::main]
async fn main() -> Result<()> { // Use eyre::Result
    // Load environment variables
    // debug_mint_aton().await?; 
    balance_of().await?;// Call the function directly
    // swap
  
    Ok(())
}