//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;


// Modules and imports
mod erc20;
mod ownable;
use alloy_sol_types::sol;


use crate::erc20::{Erc20, Erc20Params};
use alloy_primitives::{Address, U256};
use stylus_sdk::{evm,msg,contract, prelude::*};
use ownable::Ownable;

/// Immutable definitions
struct ATONParams;
impl Erc20Params for ATONParams {
    const NAME: &'static str = "ATON";
    const SYMBOL: &'static str = "STTK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct ATON {
        // Allows erc20 to access ATON's storage and make calls
        #[borrow]
        Erc20<ATONParams> erc20;
        #[borrow]
        Ownable owner;

          uint256  accumulated_commission_per_token;

  // Stores the total commission in ATON
  uint256  total_commission_in_aton;
    }
}

sol! {
    event DonateATON(address indexed sender, uint256 amount);
    event Accumulate(uint256 new_commission, uint256 accumulated, uint256 total);
    error ZeroEther(address sender);
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
}


#[public]
#[inherit(Erc20<ATONParams>,Ownable)]
impl ATON {
    /// Allows a user to donate Ether to mint ATON tokens.
    /// The Ether is converted into ATON and credited to the sender's balance.
    /// Emits a `DonateATON` event.
    pub fn donateATON(&mut self) -> Result<(), ATONError> {
        let amount = msg::value(); // Ether sent with the transaction
        let sender = msg::sender(); // Address of the sender

        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
   return Err(ATONError::ZeroEther(ZeroEther {
                sender
            }));        }
self._accumulate_commission(amount);
        // Mint equivalent ATON tokens to the sender
        self.erc20.mint(contract::address(), amount);

        
        // Emit the `DonateATON` event
        evm::log(DonateATON { sender,amount });
        Ok(())}}


impl ATON {
    /// Accumulates commission generated from swaps and stores it as ATON tokens.
    /// Updates the `accumulatedCommissionPerToken` and `totalCommissionInATON` fields.
    ///
    /// # Parameters
    /// - `new_commission_aton`: The commission amount in ATON tokens to be accumulated.
    ///
    /// # Note
    /// Assumes `total_supply()` is non-zero. If it is zero, this function will have no effect.
  pub fn _accumulate_commission(&mut self, new_commission_aton: U256) -> Result<(), ATONError> {
        let total_supply_tokens = self.erc20.total_supply();

        // Ensure no division by zero
        if total_supply_tokens > U256::from(0) {
            // Update accumulated commission per token
            let decimals = U256::from(10).pow(U256::from(ATONParams::DECIMALS));
            let additional_commission = (new_commission_aton * decimals) / total_supply_tokens;

            // Access storage fields using `.get()` and `.set()`
            let current_accumulated = self.accumulated_commission_per_token.get();
            self.accumulated_commission_per_token.set(current_accumulated + additional_commission);

            // Update total commission in ATON
            let current_total = self.total_commission_in_aton.get();
            self.total_commission_in_aton.set(current_total + new_commission_aton);

            // Emit the `Accumulate` event
            evm::log(Accumulate {
                new_commission: new_commission_aton,
                accumulated: self.accumulated_commission_per_token.get(),
                total: self.total_commission_in_aton.get(),
            });
        }

        Ok(())
    }
}



