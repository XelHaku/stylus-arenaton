//! Implementation of the ERC-20 standard
//!
//! The eponymous [`Erc20`] type provides all the standard methods,
//! and is intended to be inherited by other contract types.

//! Note that this code is unaudited and not fit for production use.

// Imported packages
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::{evm, msg, prelude::*};

// pub trait Erc20Params {
//     /// Immutable token name
//     const NAME: &'static str;

//     /// Immutable token symbol
//     const SYMBOL: &'static str;

//     /// Immutable token decimals
//     const DECIMALS: u8;
// }

sol_storage! {
    /// Erc20 implements all ERC-20 methods.
    pub struct Erc20 {

    }
}

// Declare events and Solidity error types
sol! {
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
