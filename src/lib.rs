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

/// Import items from the SDK. The prelude contains common traits and macros.

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
// sol_storage! {
//     #[entrypoint]
//     pub struct Counter {
//         uint256 number;
//     }
// }

// /// Declare that `Counter` is a contract with the following external methods.
// #[external]
// impl Counter {
//     /// Gets the number from storage.
//     pub fn number(&self) -> U256 {
//         self.number.get()
//     }

//     /// Sets a number in storage to a user-specified value.
//     pub fn set_number(&mut self, new_number: U256) {
//         self.number.set(new_number);
//     }

//     /// Sets a number in storage to a user-specified value.
//     pub fn mul_number(&mut self, new_number: U256) {
//         self.number.set(new_number * self.number.get());
//     }

//     /// Sets a number in storage to a user-specified value.
//     pub fn add_number(&mut self, new_number: U256) {
//         self.number.set(new_number + self.number.get());
//     }

//     /// Increments `number` and updates its value in storage.
//     pub fn increment(&mut self) {
//         let number = self.number.get();
//         self.set_number(number + U256::from(1));
//     }

//     /// Increments `number` and updates its value in storage.
//     pub fn decrement(&mut self) {
//         let number = self.number.get();
//         self.set_number(number - U256::from(1));
//     }
// }




// Modules and imports
mod erc20;

use crate::erc20::{Erc20, Erc20Error, Erc20Params};
use alloy_primitives::{Address, U256};
use stylus_sdk::{msg, prelude::*};

/// Immutable definitions
struct StylusTestTokenParams;
impl Erc20Params for StylusTestTokenParams {
    const NAME: &'static str = "StylusTestToken";
    const SYMBOL: &'static str = "STTK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct StylusTestToken {
        // Allows erc20 to access StylusTestToken's storage and make calls
        #[borrow]
        Erc20<StylusTestTokenParams> erc20;
    }
}

#[public]
#[inherit(Erc20<StylusTestTokenParams>)]
impl StylusTestToken {
    /// Mints tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(msg::sender(), value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(msg::sender(), value)?;
        Ok(())
    }
}