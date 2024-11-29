// src/main.rs
mod debug_mint_aton;
use debug_mint_aton::debug_mint_aton;
use eyre::Result; // Import Result from eyre

#[tokio::main]
async fn main() -> Result<()> { // Use eyre::Result
    // Load environment variables
    debug_mint_aton().await?; // Call the function directly
    // swap
  
    Ok(())
}