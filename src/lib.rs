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
use stylus_sdk::{evm,msg, prelude::*};
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
    }
}

sol! {
    event DonateATON(address indexed sender, uint256 amount);
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

        // Mint equivalent ATON tokens to the sender
        // self.erc20.mint(sender, amount)?;

        // Emit the `DonateATON` event
        evm::log(DonateATON { sender,amount });
        Ok(())}}

//           /**
//    * @dev Allows a player to donate ATON tokens to the contract. The donated amount is converted to the
//    * total commission pool, which can then be distributed to ATON holders.
//    * @notice The function requires the transaction to include Ether. The Ether is converted into ATON
//    * and credited to the contract, increasing the total ATON supply.
//    */
//   function donateATON() external payable {
//     uint256 amount = msg.value;

//     // Ensure the transaction includes some Ether to donate
//     require(amount > 0, "Must send some Ether");

//     // Mint an equivalent amount of ATON tokens to the contract address
//     _mint(address(this), amount); // Ensure _mint is correctly defined

//     // Add the donated amount to the total accumulated commission
//     _accumulateCommission(amount);

//     // Emit an event indicating that ATON tokens have been donated to the contract
//     emit EventsLib.ATONDonated(msg.sender, amount);


    // /// Mints tokens
    // pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
    //     self.erc20.mint(msg::sender(), value)?;
    //     Ok(())
    // }

    // /// Mints tokens to another address
    // pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
    //     self.erc20.mint(to, value)?;
    //     Ok(())
    // }

    // /// Burns tokens
    // pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
    //     self.erc20.burn(msg::sender(), value)?;
    //     Ok(())
    // }