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
mod constants;
// mod control;
mod erc20;
// mod ownable;
mod structs;
use alloy_sol_types::sol;

use crate::erc20::{Erc20, Erc20Error};
use alloy_primitives::{Address, U256};
// use control::AccessControl;
// use ownable::Ownable;
use stylus_sdk::{evm, msg};

use stylus_sdk::prelude::*;

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct ATON {
        // Allows erc20 to access ATON's storage and make calls
        #[borrow]
        Erc20 erc20;
        // #[borrow]
        // Ownable ownable;

        // #[borrow]
        //       AccessControl access;


          uint256  accumulated_commission_per_token;

  // Stores the total commission in ATON
  uint256  total_commission_in_aton;
    mapping(address => uint256) last_commission_per_token;
    mapping(address => uint256) claimed_commissions;



    }
}

sol! {
    event DonateATON(address indexed sender, uint256 amount);
    event Accumulate(uint256 new_commission, uint256 accumulated, uint256 total);
    error ZeroEther(address sender);
    error ZeroAton(address sender);

     #[derive(AbiType)]
    struct Foo {
        uint256 bar;
    }
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
}

#[public]
#[inherit(Erc20)]
impl ATON {
    pub fn debug_mint_aton(&mut self) -> Result<bool, Vec<u8>> {
        let _ = self.erc20.mint(msg::sender(), msg::value());
        Ok(true)
    }
    //     // pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
    //     //     self.erc20._transfer(msg::sender(), to, value)?; // _
    //     //     Ok(true)
    //     // }
    //     /// Allows a user to donate Ether to mint ATON tokens.
    //     /// The Ether is converted into ATON and credited to the sender's balance.
    //     /// Emits a `DonateATON` event.
    //     pub fn donate_aton(&mut self) -> Result<bool,ATONError> {
    //         let amount = msg::value(); // Ether sent with the transaction
    //         let sender = msg::sender(); // Address of the sender

    //         // Ensure the transaction includes some Ether to donate
    //         if amount == U256::from(0) {
    //             return Err(
    //                 ATONError::ZeroEther(ZeroEther {
    //                     sender,
    //                 })
    //             );
    //         }
    //         let _ = self._accumulate_commission(amount);
    //         // Mint equivalent ATON tokens to the sender
    //         let _ = self.erc20.mint(contract::address(), amount);

    //         // Emit the `DonateATON` event
    //         evm::log(DonateATON { sender, amount });
    //         Ok(true)
    //     }

    //     pub fn stake_eth(&mut self, _player: Address) -> Result<bool, Vec<u8>> {
    //         self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
    //         let _ = self.erc20.mint(contract::address(), msg::value());
    //         Ok(true)
    //     }

    //     pub fn stake_aton(&mut self, _player: Address, _amount: U256) -> Result<bool, Vec<u8>> {
    //         let _ = self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
    //         let _ = self.erc20.transfer_from(_player, contract::address(), _amount);
    //         Ok(true)
    //     }

    //     pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
    //         if amount == U256::from(0) {
    //  return Err(
    //                 ATONError::ZeroAton(ZeroAton {
    //                     sender: msg::sender(),
    //                 })
    //             );        }
    //         let balance_aton = self.erc20.balance_of(msg::sender());

    //         if balance_aton < amount {
    //             return Ok(true); // error
    //         }
    //         let balance_eth = contract::balance();

    //         if balance_eth < amount {
    //             return Ok(true); // error
    //         }

    //         let _ = transfer_eth(msg::sender(), amount); // these two are equivalent

    //         // let _ = self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
    //         // let _ = self.erc20.transfer_from(_player,contract::address(), _amount);
    //         Ok(true)
    //     }

    //         pub fn player_summary(&mut self, player: Address) -> Result<bool, ATONError> {
    // let _player_commission = self._player_commission(player);
    //      Ok(true)
    //     }

    //       /**
    //    * @dev Retrieves a summary of a single player's data and includes global commission data,
    //    * as well as a batch of event IDs (either active or closed).
    //    * @param playerAddress The address of the player.
    //    * @return summary A PlayerSummary struct containing the player's summary data.
    //    * @return totalCommission The total commission in ATON.
    //    * @return accumulatedCommission The accumulated commission per token.
    //    */
    //   function playerSummary(
    //     address playerAddress
    //   )
    //     external
    //     view
    //     returns (AStructs.PlayerSummary memory summary, uint256 totalCommission, uint256 accumulatedCommission)
    //   {
    //     AStructs.Player storage player = players[playerAddress];

    //     // Populate the player's summary
    //     summary = AStructs.PlayerSummary({
    //       level: player.level, // Player's current level
    //       ethBalance: playerAddress.balance, // Player's ETH balance
    //       atonBalance: balanceOf(playerAddress), // Player's ATON token balance
    //       unclaimedCommission: _playerCommission(playerAddress), // Player's unclaimed commission
    //       claimedCommission: player.claimedCommissionsByPlayer // Player's total claimed commission
    //     });

    //     // Assign the global data to the return values
    //     totalCommission = totalCommissionInATON;
    //     accumulatedCommission = accumulatedCommissionPerToken;

    //     // Return the player's summary along with the global commission data
    //     return (summary, totalCommission, accumulatedCommission);
    //   }
}

//      /**
//    * @dev Swaps ATON tokens for ETH at a 1:1 ratio.
//    * @param _amountAton The amount of ATON tokens to swap.
//    * @return success True if the swap was successful.
//    */
//   function swap(uint256 _amountAton) external nonReentrant returns (bool success) {
//     require(_amountAton > 0, "Swap amount must be greater than zero");
//     require(balanceOf(msg.sender) >= _amountAton, "Insufficient ATON balance");
//     require(address(this).balance >= _amountAton, "Contract has insufficient ETH balance");

//     // Step 1: Transfer ATON tokens to the contract
//     _distributeTransfer(msg.sender, address(this), _amountAton);

//     // Step 2: Burn the ATON tokens from the contract to maintain the 1:1 swap mechanism
//     _burn(address(this), _amountAton);

//     // Step 3: Transfer Ether to the sender (after state changes)
//     (bool sent, ) = msg.sender.call{ value: _amountAton }("");
//     require(sent, "Failed to send ETH");

//     // Emit the swap event after successful transfer
//     emit EventsLib.Swap(msg.sender, _amountAton);

//     return true;
//   }

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
            self.accumulated_commission_per_token
                .set(current_accumulated + additional_commission);

            // Update total commission in ATON
            let current_total = self.total_commission_in_aton.get();
            self.total_commission_in_aton
                .set(current_total + new_commission_aton);

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
    pub fn _player_commission(&mut self, player: Address) -> Result<U256, Erc20Error> {
        let pct_denom: U256 = U256::from(10000000);

        let _owed_per_token = self.accumulated_commission_per_token.get()
            - self.last_commission_per_token.get(player);
        let _unclaimed_commission = (self.erc20.balance_of(player) * _owed_per_token * pct_denom)
            / U256::from(10).pow(U256::from(18u8));
        Ok(_unclaimed_commission)
    }

    /*************  ✨ Codeium Command 🌟  *************/
    //     pub fn distribute_commission(&mut self, player: Address) -> Result<U256, Erc20Error> {
    //         let unclaimed_commission = self._player_commission(player);
    //                             if unclaimed_commission > U256::from(0) {
    //             // Update claimed commissions

    //             self.transfer(player, unclaimed_commission); // transfer(address from, address to, value)
    //   let _cc = self.claimed_commissions.get(player);
    //             self.claimed_commissions.insert(player, _cc + unclaimed_commission);

    //         }
}
