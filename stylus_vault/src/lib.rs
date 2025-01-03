// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

// Modules and imports

use stylus_sdk::{alloy_sol_types::sol,call::transfer_eth, contract, evm, msg,alloy_primitives::{Address, U256}, call::{ call, Call },};
use stylus_sdk::prelude::*;
// use alloy_sol_macro::sol;
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Vault {
        uint256  accumulated_commission_per_token;
        uint256  total_commission_in_aton;
        mapping(address => uint256) last_commission_per_token;
        mapping(address => uint256) claimed_commissions;
address aton_address;

    }





}


sol_interface! {
    interface IATON {

    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function balanceOf(address owner) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function totalSupply() external view returns (uint256);


}
}

sol! {


    // Vault
    event CommissionAccumulate(uint256 indexed amount, uint256 indexed newAccPerToken, uint256 indexed totalCommission);

    error Zero(address account);

        // Access Control
    // event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    // event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);


    // Ownership
    error UnauthorizedAccount(address account);

}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum VaultError {
    Zero(Zero),
    UnauthorizedAccount(UnauthorizedAccount),
}

#[public]
impl Vault {


//     fn aton_address(&self) -> Address {
//         self.aton_address.get()
//     }

//     pub fn initialize(&mut self, _aton_address: Address) -> bool{
//         if self.aton_address.get() != Address::ZERO {
// return false;        }
//         self.aton_address.set(_aton_address);
//         true
//     }




//     pub fn accumulate_aton(&mut self, amount: U256) -> Result<(bool), VaultError> {
//         // Ensure the transaction includes some Ether to donate
//         if amount == U256::from(0) {
//                      return Err(VaultError::Zero(Zero {account: msg::sender()}));// Add the error struct

//         }

//                  let aton_contract = IATON::new(self.aton_address.get());
//             let config = Call::new_in(self);

//     // Convert the error returned by `is_oracle` to `VaultError`
//     let _ = aton_contract
//         .transfer_from(config, msg::sender(),contract::address(), amount)
//         .map_err(|_| VaultError::Zero(Zero {account: msg::sender()}))?;

//         let _ = self.add_commission(amount);

//         evm::log(CommissionAccumulate {
//             amount,
//             newAccPerToken: self.accumulated_commission_per_token.get(),
//             totalCommission: self.total_commission_in_aton.get(),
//         });
//         Ok(true)
//     }



//     pub fn summary(&mut self, player: Address) -> Result<(U256, U256, U256), VaultError> {
//         Ok((
//             self._player_commission(player),
//             self.claimed_commissions.get(player),
//             *self.total_commission_in_aton,
//         ))
//     }



//     fn handle_commissions(&mut self, caller: Address, to: Address) {
//         // Distribute commission to the caller
//         if caller != contract::address() {
//             self.distribute_commission(caller);
//         }

//         // Distribute commission to the recipient
//         if to != contract::address() {
//             self.distribute_commission(to);
//         }

//         // If either party is the contract, distribute commission to the owner
//         if caller == contract::address() || to == contract::address() {
//             self.distribute_commission(self.aton_address.get());
//         }
//     }

}

// Private Functions
impl Vault { 
    
    // Ownable 
    pub fn only_owner(&self) -> Result<(), VaultError> {
        let account = msg::sender();
        if self.aton_address.get() != account {
            return Err(VaultError::UnauthorizedAccount(
                UnauthorizedAccount { account },
            ));
        }

        Ok(())
    }

 pub fn add_commission(&mut self, new_commission_aton: U256) -> Result<(), VaultError> {
    let aton_contract = IATON::new(self.aton_address.get());
    let config = Call::new_in(self);

    // Retrieve the total supply of tokens
    let total_supply_tokens = aton_contract
        .total_supply(config)
        .map_err(|_| VaultError::Zero(Zero { account: msg::sender() }))?;

    // Ensure no division by zero
    if total_supply_tokens > U256::from(0) {
        // Update accumulated commission per token
        let additional_commission =
            (new_commission_aton * U256::from(10).pow(U256::from(18u8))) / total_supply_tokens;

        // Access storage fields using `.get()` and `.set()`
        self.accumulated_commission_per_token
            .set(self.accumulated_commission_per_token.get() + additional_commission);

        // Update total commission in Vault
        self.total_commission_in_aton
            .set(self.total_commission_in_aton.get() + new_commission_aton);
    }

    Ok(())
}
/// Returns the unclaimed commisfor a player
pub fn _player_commission(&self, player: Address) -> U256 {
    // 1) Figure out how much is owed per token since last time
    let owed_per_token = self
        .accumulated_commission_per_token
        .saturating_sub(self.last_commission_per_token.get(player));

    // 2) Multiply that by player balance

    let decimals = U256::from(10).pow(U256::from(18));
    let pct_denom = U256::from(10000000u64);

    // Instantiate the ATON contract interface
    let aton_contract = IATON::new(self.aton_address.get());

    // Attempt to get the player's ATON balance
    let player_aton_balance = match aton_contract.balance_of(self, msg::sender()) {
        Ok(balance) => balance,
        Err(_) => {
            // Log or handle the error if necessary
            return U256::ZERO;
        }
    };

    // Perform calculations
    let scaled = player_aton_balance
        .checked_mul(owed_per_token)
        .unwrap_or(U256::ZERO)
        .checked_mul(pct_denom)
        .unwrap_or(U256::ZERO)
        / decimals;

    // Return the final scaled value or zero if scaled is zero
    if scaled > U256::ZERO {
        scaled / pct_denom
    } else {
        U256::ZERO
    }
}

pub fn distribute_commission(&mut self, player: Address) -> Result<(), VaultError> {
    let unclaimed = self._player_commission(player);

    if unclaimed > U256::ZERO {
        let pay_to = if player == contract::address() {
            self.aton_address.get()
        } else {
            player
        };

        // Instantiate the ATON contract interface
        let aton_contract = IATON::new(self.aton_address.get());


        // Check contract's ATON balance
        let contract_aton_balance = aton_contract
            .balance_of(Call::new_in(self), contract::address())
            .map_err(|_| VaultError::Zero(Zero { account: msg::sender() }))?;

        // Perform transfer if balance is sufficient
        if contract_aton_balance >= unclaimed {
            aton_contract
                .transfer(Call::new_in(self), pay_to, unclaimed)
                .map_err(|_| VaultError::Zero(Zero { account: msg::sender() }))?;
        }
    }

    // Update last_commission_per_token after operations
    self.last_commission_per_token
        .setter(player)
        .set(self.accumulated_commission_per_token.get());

    Ok(())
}







}
