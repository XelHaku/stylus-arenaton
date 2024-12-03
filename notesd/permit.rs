//! Permit Contract.
//!
//! Extension of the ERC-20 standard allowing approvals to be made
//! via signatures, as defined in EIP-2612.
//!
//! Adds the `permit` method, which can be used to change an account’s
//! ERC20 allowance (see [`crate::token::erc20::IErc20::allowance`])
//! by presenting a message signed by the account.
//! By not relying on [`crate::token::erc20::IErc20::approve`],
//! the token holder account doesn’t need to send a transaction,
//! and thus is not required to hold Ether at all.
use alloy_primitives::{b256, keccak256, Address, B256, U256};
use alloy_sol_types::{sol, SolType};
use stylus_sdk::{
    block,
    prelude::StorageType,
    storage::TopLevelStorage,
    stylus_proc::{public, sol_storage, SolidityError},
};

use crate::{
    token::erc20::{self, Erc20, IErc20},
    utils::{
        cryptography::{ecdsa, eip712::IEip712},
        nonces::Nonces,
    },
};

// keccak256("Permit(address owner,address spender,uint256 value,uint256
// nonce,uint256 deadline)")
const PERMIT_TYPEHASH: B256 =
    b256!("6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9");

type StructHashTuple = sol! {
    tuple(bytes32, address, address, uint256, uint256, uint256)
};

sol! {
    /// Indicates an error related to the fact that
    /// permit deadline has expired.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC2612ExpiredSignature(uint256 deadline);

    /// Indicates an error related to the issue about mismatched signature.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC2612InvalidSigner(address signer, address owner);
}

/// A Permit error.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// Indicates an error related to the fact that
    /// permit deadline has expired.
    ExpiredSignature(ERC2612ExpiredSignature),
    /// Indicates an error related to the issue about mismatched signature.
    InvalidSigner(ERC2612InvalidSigner),
    /// Error type from [`Erc20`] contract [`erc20::Error`].
    Erc20(erc20::Error),
    /// Error type from [`ecdsa`] contract [`ecdsa::Error`].
    ECDSA(ecdsa::Error),
}

sol_storage! {
    /// State of a Permit Contract.
    pub struct Erc20Permit<T: IEip712 + StorageType>{
        /// ERC-20 contract.
        Erc20 erc20;

        /// Nonces contract.
        Nonces nonces;

        /// EIP-712 contract. Must implement [`IEip712`] trait.
        T eip712;
    }
}


unsafe impl<T: IEip712 + StorageType> TopLevelStorage for Erc20Permit<T> {}

#[public]
impl<T: IEip712 + StorageType> Erc20Permit<T> {

    #[must_use]
    pub fn nonces(&self, owner: Address) -> U256 {
        self.nonces.nonces(owner)
    }

    #[selector(name = "DOMAIN_SEPARATOR")]
    #[must_use]
    pub fn domain_separator(&self) -> B256 {
        self.eip712.domain_separator_v4()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn permit(
        &mut self,
        owner: Address,
        spender: Address,
        value: U256,
        deadline: U256,
        v: u8,
        r: B256,
        s: B256,
    ) -> Result<(), Error> {
        if U256::from(block::timestamp()) > deadline {
            return Err(ERC2612ExpiredSignature { deadline }.into());
        }

        let struct_hash = keccak256(StructHashTuple::abi_encode(&(
            *PERMIT_TYPEHASH,
            owner,
            spender,
            value,
            self.nonces.use_nonce(owner),
            deadline,
        )));

        let hash: B256 = self.eip712.hash_typed_data_v4(struct_hash);

        let signer: Address = ecdsa::recover(self, hash, v, r, s)?;

        if signer != owner {
            return Err(ERC2612InvalidSigner { signer, owner }.into());
        }

        self.erc20._approve(owner, spender, value, true)?;

        Ok(())
    }


}