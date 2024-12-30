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
mod erc20;
use crate::erc20::{ Erc20, Erc20Error };


mod control;
use crate::control::AccessControl;
// Modules and imports
mod constants;
// mod ownable;
mod structs;
use alloy_sol_types::sol;

use alloy_primitives::{ Address, U256 };

use stylus_sdk::{
    call::transfer_eth,
    contract,
    evm,
    msg,
    stylus_proc::{ public, sol_storage, SolidityError },
};
use alloy_primitives::FixedBytes;

use stylus_sdk::prelude::*;

// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ATON {
                #[borrow]
        Erc20 erc20;

#[borrow]
AccessControl control;
        uint256 number;


          uint256  accumulated_commission_per_token;

  // Stores the total commission in ATON
  uint256  total_commission_in_aton;
    mapping(address => uint256) last_commission_per_token;
    mapping(address => uint256) claimed_commissions;


        bool initialized ;
address owner;

    }
}

sol! {
   

    // ATON
    event DonateATON(address indexed sender, uint256 amount);
    event Accumulate(uint256 new_commission, uint256 accumulated, uint256 total);
    error ZeroEther(address sender);
    error ZeroAton(address sender);
    error AlreadyInitialized();
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
    AlreadyInitialized(AlreadyInitialized),
}

#[public]
#[inherit(Erc20, AccessControl)]
impl ATON {
    pub fn initialize_contract(&mut self) -> Result<bool, ATONError> {
        if self.initialized.get() {
            // Access the value using .get()
            return Err(ATONError::AlreadyInitialized(AlreadyInitialized {})); // Add the error struct
        }
        self.initialized.set(true); // Set initialized to true
        self.owner.set(msg::sender());
        self.control._grant_role(FixedBytes::from(constants::DEFAULT_ADMIN_ROLE), msg::sender());
        Ok(true)
    }

    #[payable]
    pub fn donate_eth(&mut self) -> Result<bool, ATONError> {
        let amount = msg::value(); // Ether sent with the transaction
        let sender = msg::sender(); // Address of the sender

        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
            return Err(ATONError::ZeroEther(ZeroEther { sender }));
        }
        let _ = self._accumulate_commission(amount);
        // Mint equivalent ATON tokens to the sender
        let _ = self.erc20.mint(contract::address(), amount);

        // Emit the `DonateATON` event
        evm::log(DonateATON { sender, amount });
        Ok(true)
    }

     pub fn accumulate_aton(&mut self, amount: U256) -> Result<bool, ATONError> {
        let sender = msg::sender(); // Address of the sender

        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
            return Err(ATONError::ZeroAton  (ZeroAton  { sender }));
        }
        let _ = self._accumulate_commission(amount);
        // Mint equivalent ATON tokens to the sender
        let _ = self.erc20.transfer(contract::address(), amount);

        // Emit the `DonateATON` event
        evm::log(DonateATON { sender, amount });
        Ok(true)
    }

    #[payable]
    pub fn mint_aton_from_eth(&mut self) -> Result<bool, Vec<u8>> {
        self.control.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
        let _ = self.erc20.mint(msg::sender(), msg::value());

        Ok(true)
    }

    pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
        if amount == U256::from(0) {
            return Err(
                ATONError::ZeroAton(ZeroAton {
                    sender: msg::sender(),
                })
            );
        }
        let balance_aton = self.erc20.balance_of(msg::sender());

        if balance_aton < amount {
            return Err(
                ATONError::ZeroAton(ZeroAton {
                    sender: msg::sender(),
                })
            );
        }
        let balance_eth = contract::balance();

        if balance_eth < amount {
            return Ok(true); // error
        }

        let _ = transfer_eth(msg::sender(), amount); // these two are equivalent

        Ok(true)
    }

    pub fn summary(&mut self) -> Result<(U256, U256, U256), ATONError> {
        let player_commission = self._player_commission(msg::sender())?;

        let player_claimed = self.claimed_commissions.get(msg::sender());
        Ok((player_commission, *self.total_commission_in_aton, player_claimed))
    }
}

// Private Functions
impl ATON {
    /// Accumulates commission generated from swaps and stores it as ATON tokens.
    /// Updates the `accumulated_commission_per_token` and `totalCommissionInATON` fields.
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
            let decimals = U256::from(10).pow(U256::from(18u8));
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
    //       /**
    //    * @dev Computes the unclaimed commission for a specified player based on their ATON token holdings.
    //    * @param player Address of the player.
    //    * @return unclaimedCommission The amount of ATON tokens the player can claim as commission.
    //    * @notice The calculation is based on the difference between the global accumulated commission per token
    //    * and the player's last recorded commission per token, scaled by the player's ATON holdings and adjusted by `pct_denom` for precision.
    //    */
    pub fn _player_commission(&mut self, player: Address) -> Result<U256, ATONError> {
        let pct_denom: U256 = U256::from(10000000);

        let _owed_per_token =
            self.accumulated_commission_per_token.get() -
            self.last_commission_per_token.get(player);
        let _unclaimed_commission =
            (self.erc20.balance_of(player) * _owed_per_token * pct_denom) /
            U256::from(10).pow(U256::from(18u8));
        Ok(_unclaimed_commission)
    }
}
