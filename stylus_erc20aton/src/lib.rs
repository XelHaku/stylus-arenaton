// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

// Modules and imports

// use alloy_sol_types::sol;
use stylus_sdk::{alloy_sol_types::sol,call::transfer_eth, contract, evm, msg,alloy_primitives::{Address, U256}};

use stylus_sdk::prelude::*;

// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ATON {
        bool initialized ;


        uint256  accumulated_commission_per_token;
        uint256  total_commission_in_aton;
        uint256  current_pot;
        mapping(address => uint256) last_commission_per_token;
        mapping(address => uint256) claimed_commissions;


        address owner;
                            /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;


    }





}

sol! {


    // ATON
    event DonateATON(address indexed sender, uint256 indexed amount);
    event CommissionAccumulate(uint256 indexed amount, uint256 indexed newAccPerToken, uint256 indexed totalCommission);

    error Zero(address account);

        // Access Control
    // event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    // event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);


    // Ownership
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    error UnauthorizedAccount(address account);

     // ERC20
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    InsufficientBalance(InsufficientBalance),
    Zero(Zero),
    InsufficientAllowance(InsufficientAllowance),
    UnauthorizedAccount(UnauthorizedAccount),
}

#[public]
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


    fn owner(&self) -> Address {
        self.owner.get()
    }

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), ATONError> {
        self.only_owner()?;

        if new_owner.is_zero() {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account: Address::ZERO,
            }));
        }

        self._transfer_ownership(new_owner);

        Ok(())
    }
    pub fn initialize(&mut self) -> Result<bool, ATONError> {
        if self.initialized.get() {
            // Access the value using .get()
            return Err(ATONError::Zero(Zero {account: msg::sender()})); // Add the error struct
        }
        self.initialized.set(true); // Set initialized to true
        self.owner.set(msg::sender());
        Ok(true)
    }



    #[payable]
    pub fn mint_aton(&mut self) -> Result<bool, ATONError> {
        // let is_engine = self._has_role(constants::ARENATON_ENGINE_ROLE.into(), msg::sender());
     let is_engine = true;
        if is_engine == false {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account: msg::sender(),
            }));
        }

        let _ = self.mint(msg::sender(), msg::value());

        Ok(true)
    }

    pub fn accumulate_aton(&mut self, amount: U256) -> Result<bool, ATONError> {
        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
                     return Err(ATONError::Zero(Zero {account: msg::sender()})); // Add the error struct

        }
        let _ = self.transfer(contract::address(), amount);
        let _ = self.add_commission(amount);

        evm::log(CommissionAccumulate {
            amount,
            newAccPerToken: self.accumulated_commission_per_token.get(),
            totalCommission: self.total_commission_in_aton.get(),
        });
        Ok(true)
    }
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, ATONError> {
        let caller = msg::sender();

        // Distribute commissions
        self.handle_commissions(caller, to);

        // Perform the transfer
        self
            ._transfer(caller, to, amount)
            .map(|_| true)
            .map_err(|_| ATONError::InsufficientBalance(InsufficientBalance {                 from: msg::sender(),want: amount, have: self.balance_of(msg::sender())
 }))
    }

pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
    let sender = msg::sender();
    // let sender_balance = self.balance_of(sender);
       let sender_balance =  self.balances.get(sender);

    let contract_balance = contract::balance();

    if amount == U256::from(0) || sender_balance < amount || contract_balance < amount {
            return Err(ATONError::Zero(Zero {account: msg::sender()})); // Add the error struct

    }
        let _ = transfer_eth(sender, amount);

    Ok(true)
}


    // pub fn summary(&mut self, player: Address) -> Result<(U256, U256, U256), ATONError> {
    //     Ok((
    //         self._player_commission(player),
    //         self.players.get(player).claimed_commissions.get(),
    //         *self.total_commission_in_aton,
    //     ))
    // }




}

// Private Functions
impl ATON { 
    
    // Ownable 
    pub fn only_owner(&self) -> Result<(), ATONError> {
        let account = msg::sender();
        if self.owner.get() != account {
            return Err(ATONError::UnauthorizedAccount(
                UnauthorizedAccount { account },
            ));
        }

        Ok(())
    }

    pub fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self.owner.get();
        self.owner.set(new_owner);
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }

    // Commission Management
    fn handle_commissions(&mut self, caller: Address, to: Address) {
        // Distribute commission to the caller
        if caller != contract::address() {
            self.distribute_commission(caller);
        }

        // Distribute commission to the recipient
        if to != contract::address() {
            self.distribute_commission(to);
        }

        // If either party is the contract, distribute commission to the owner
        if caller == contract::address() || to == contract::address() {
            self.distribute_commission(self.owner.get());
        }
    }

    pub fn add_commission(&mut self, new_commission_aton: U256) -> Result<(), ATONError> {
        let total_supply_tokens = self.total_supply();

        // Ensure no division by zero
        if total_supply_tokens > U256::from(0) {
            // Update accumulated commission per token
            let additional_commission =
                (new_commission_aton * U256::from(10).pow(U256::from(18u8))) / total_supply_tokens;

            // Access storage fields using `.get()` and `.set()`
            self.accumulated_commission_per_token
                .set(self.accumulated_commission_per_token.get() + additional_commission);

            // Update total commission in ATON
            self.total_commission_in_aton
                .set(self.total_commission_in_aton.get() + new_commission_aton);
        }

        Ok(())
    }

  
    /// Returns the unclaimed commission for a player
    pub fn _player_commission(&self, player: Address) -> U256 {
        // 1) Figure out how much is owed per token since last time
        let owed_per_token = self
            .accumulated_commission_per_token
            .saturating_sub(self.last_commission_per_token.get(player));

        // 2) Multiply that by player balance
       let balance =  self.balances.get(player);

        let decimals = U256::from(10).pow(U256::from(18));
        // Optional extra precision factor (pct_denom)
        let pct_denom = U256::from(10000000u64);

        let scaled = balance
            .checked_mul(owed_per_token)
            .unwrap_or(U256::ZERO)
            .checked_mul(pct_denom)
            .unwrap_or(U256::ZERO)
            / decimals;

        // The final value is scaled / pct_denom
        if scaled > U256::ZERO {
            scaled / pct_denom
        } else {
            U256::ZERO
        }
    }

    /// Pays out the unclaimed commission to the given player (or to owner if player == contract).
pub fn distribute_commission(&mut self, player: Address) {
    let unclaimed = self._player_commission(player);

    if unclaimed > U256::ZERO {
        let pay_to = if player == contract::address() {
            self.owner.get()
        } else {
            player
        };

        // Check balance and perform transfer in a separate block
        if self.balance_of(contract::address()) >= unclaimed {
            // Perform the transfer
            if let Ok(_) = self._transfer(contract::address(), pay_to, unclaimed) {
                // Update claimed commissions in a separate scope to avoid mutable borrow overlap
                {
                    let mut claimed_commissions = self.claimed_commissions.setter(player);
                    let _claimed = claimed_commissions.get();
                    claimed_commissions.set(unclaimed + _claimed);
                }
            }
        }
    }

    // Update last_commission_per_token after mutable borrows are dropped
    self.last_commission_per_token.setter(player)
        .set(self.accumulated_commission_per_token.get());
}

// Token Management   
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
}
