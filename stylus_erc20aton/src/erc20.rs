// Imported packages
use stylus_sdk::{alloy_sol_types::sol,evm, msg, prelude::*,alloy_primitives::{Address, U256}};

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ERC20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}
sol_storage! {
    /// Erc20 implements all ERC-20 methods.
    pub struct Erc20 {
                    /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;

    }
}

// Declare events and Solidity error types
sol! {
    // ERC20
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);

}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

// These methods aren't exposed to other contracts
// Methods marked as "pub" here are usable outside of the erc20 module (i.e. they're callable from lib.rs)
// Note: modifying storage will become much prettier soon
impl Erc20 {
   
}

// These methods are public to other contracts
// Note: modifying storage will become much prettier soon
#[public]
impl Erc20 {
  
}
