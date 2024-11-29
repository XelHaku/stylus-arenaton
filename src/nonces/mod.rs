//! Implementation of nonce tracking for addresses.
//!
//! Nonces will only increment.

use alloy_primitives::{uint, Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::stylus_proc::{public, sol_storage, SolidityError};

const ONE: U256 = uint!(1_U256);

sol! {
    /// The nonce used for an `account` is not the expected current nonce.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error InvalidAccountNonce(address account, uint256 currentNonce);
}

/// A Nonces error.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// The nonce used for an `account` is not the expected current nonce.
    InvalidAccountNonce(InvalidAccountNonce),
}

sol_storage! {
    /// State of a Nonces Contract.
    pub struct Nonces {
        /// Mapping from address to its nonce.
        mapping(address => uint256) _nonces;
    }
}

#[public]
impl Nonces {
    /// Returns the unused nonce for the given account.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `owner` - The address for which to return the nonce.
    #[must_use]
    pub fn nonces(&self, owner: Address) -> U256 {
        self._nonces.get(owner)
    }
}

impl Nonces {
    /// Consumes a nonce for the given `account`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `owner` - The address for which to consume the nonce.
    ///
    /// # Panics
    ///
    /// This function will panic if the nonce for the given `owner` has reached
    /// the maximum value representable by `U256`, causing the `checked_add`
    /// method to return `None`.
    pub fn use_nonce(&mut self, owner: Address) -> U256 {
        let nonce = self._nonces.get(owner);
        self._nonces
            .setter(owner)
            .set(unsafe { nonce.checked_add(ONE).unwrap_unchecked() });

        nonce
    }

    /// Same as `use_nonce` but checking that the `nonce` is the next valid for
    /// the owner.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `owner` - The address for which to consume the nonce.
    /// * `nonce` - The nonce to consume.
    ///
    /// # Panics
    ///
    /// This function will panic if the nonce for the given `owner` has reached
    /// the maximum value representable by `U256`, causing the `checked_add`
    /// method to return `None`.
    ///
    /// # Errors
    ///
    /// Returns an error if the `nonce` is not the next valid nonce for the
    /// owner.
    pub fn use_checked_nonce(
        &mut self,
        owner: Address,
        nonce: U256,
    ) -> Result<(), Error> {
        let current_nonce = self._nonces.get(owner);

        if nonce != current_nonce {
            return Err(Error::InvalidAccountNonce(InvalidAccountNonce {
                account: owner,
                currentNonce: current_nonce,
            }));
        }

        self._nonces
            .setter(owner)
            .set(unsafe { nonce.checked_add(ONE).unwrap_unchecked() });

        Ok(())
    }
}

