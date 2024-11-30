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
mod control;
// mod ownable;
mod structs;
use alloy_sol_types::sol;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ contract, evm, msg };
use stylus_sdk::call::transfer_eth;
use stylus_sdk::prelude::*;

use control::AccessControl;


// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct ATON {
        #[borrow]
        AccessControl access;


          uint256  accumulated_commission_per_token;

  // Stores the total commission in ATON
  uint256  total_commission_in_aton;
    mapping(address => uint256) last_commission_per_token;
    mapping(address => uint256) claimed_commissions;
            /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;

        bool initialized ;



    }
}

sol! {
    // ERC20 
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
    
    // ATON 
    event DonateATON(address indexed sender, uint256 amount);
    event Accumulate(uint256 new_commission, uint256 accumulated, uint256 total);
    error ZeroEther(address sender);
    error ZeroAton(address sender);

 

}



/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

#[public]
#[inherit(AccessControl)]
impl ATON {


     /// Immutable token name
    pub fn name() -> String {
        "ATON Stylus".into()
    }

    /// Immutable token symbol
    pub fn symbol() -> String {
        "ATON".into()
    }

    /// Immutable token decimals
    pub fn decimals() -> u8 {
        18u8
    }

    /// Total supply of tokens
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    /// Balance of `address`
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    /// Transfers `value` tokens from msg::sender() to `to`
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, ATONError> {
        self._transfer(msg::sender(), to, value)?;
        Ok(true)
    }

    /// Transfers `value` tokens from `from` to `to`
    /// (msg::sender() must be able to spend at least `value` tokens from `from`)
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, ATONError> {
        // Check msg::sender() allowance
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(ATONError::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: msg::sender(),
                have: old_allowance,
                want: value,
            }));
        }

        // Decreases allowance
        allowance.set(old_allowance - value);

        // Calls the internal transfer function
        self._transfer(from, to, value)?;

        Ok(true)
    }

    /// Approves the spenditure of `value` tokens of msg::sender() to `spender`
    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.allowances.setter(msg::sender()).insert(spender, value);
        evm::log(Approval {
            owner: msg::sender(),
            spender,
            value,
        });
        true
    }

    /// Returns the allowance of `spender` on `owner`'s tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }
    #[payable]
    pub fn debug_mint_aton(&mut self) -> Result<bool, ATONError> {
        let _ = self.mint(msg::sender(), msg::value());
        Ok(true)
    }


    #[payable]
    pub fn donate_eth(&mut self) -> Result<bool, ATONError> {
        let amount = msg::value(); // Ether sent with the transaction
        let sender = msg::sender(); // Address of the sender

        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
            return Err(
                ATONError::ZeroEther(ZeroEther {
                    sender,
                })
            );
        }
        let _ = self._accumulate_commission(amount);
        // Mint equivalent ATON tokens to the sender
        let _ = self.mint(contract::address(), amount);

        // Emit the `DonateATON` event
        evm::log(DonateATON { sender, amount });
        Ok(true)
    }

    #[payable]
    pub fn stake_eth(&mut self, _player: Address) -> Result<bool, Vec<u8>> {
        // self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
        let _ = self.mint(contract::address(), msg::value());
        Ok(true)
    }

    pub fn stake_aton(&mut self, _player: Address, _amount: U256) -> Result<bool, Vec<u8>> {
        // let _ = self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
        let _ = self.transfer_from(_player, contract::address(), _amount);
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
        let balance_aton = self.balance_of(msg::sender());

        if balance_aton < amount {
            return Ok(true); // error
        }
        let balance_eth = contract::balance();

        if balance_eth < amount {
            return Ok(true); // error
        }

        let _ = transfer_eth(msg::sender(), amount); // these two are equivalent

        // let _ = self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
        // let _ = self.transfer_from(_player,contract::address(), _amount);
        Ok(true)
    }

}


// Private Functions
impl ATON {

        /// Movement of funds between 2 accounts
    /// (invoked by the public transfer() and transfer_from() functions )
    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), ATONError> {
        // Decreasing sender balance
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(ATONError::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }
        sender_balance.set(old_sender_balance - value);

        // Increasing receiver balance
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);

        // Emitting the transfer event
        evm::log(Transfer { from, to, value });
        Ok(())
    }

    /// Mints `value` tokens to `address`
    pub fn mint(&mut self, address: Address, value: U256) -> Result<(), ATONError> {
        // Increasing balance
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);

        // Increasing total supply
        self.total_supply.set(self.total_supply.get() + value);

        // Emitting the transfer event
        evm::log(Transfer {
            from: Address::ZERO,
            to: address,
            value,
        });

        Ok(())
    }


    /// Accumulates commission generated from swaps and stores it as ATON tokens.
    /// Updates the `accumulated_commission_per_token` and `totalCommissionInATON` fields.
    ///
    /// # Parameters
    /// - `new_commission_aton`: The commission amount in ATON tokens to be accumulated.
    ///
    /// # Note
    /// Assumes `total_supply()` is non-zero. If it is zero, this function will have no effect.
    pub fn _accumulate_commission(&mut self, new_commission_aton: U256) -> Result<(), ATONError> {
        let total_supply_tokens = self.total_supply();

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
            (self.balance_of(player) * _owed_per_token * pct_denom) /
            U256::from(10).pow(U256::from(18u8));
        Ok(_unclaimed_commission)
    }

    }
