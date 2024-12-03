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
// mod ownable;
mod structs;
use alloy_sol_types::sol;

use alloy_primitives::{Address, B256, U256};

use stylus_sdk::{
    call::transfer_eth,
    contract, evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

use alloy_primitives::FixedBytes;


use stylus_sdk::prelude::*;

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct ATON {



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


        /// Role identifier -> Role information.
        mapping(bytes32 => RoleData) _roles;

        address owner;
    }

        /// Information about a specific role.
    pub struct RoleData {
        /// Whether an account is member of a certain role.
        mapping(address => bool) has_role;
        /// The admin role for this role.
        bytes32 admin_role;
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

sol! {
    /// Emitted when `new_admin_role` is set as `role`'s admin role, replacing
    /// `previous_admin_role`.
    ///
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// `RoleAdminChanged` not being emitted signaling this.
    #[allow(missing_docs)]
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previous_admin_role, bytes32 indexed new_admin_role);
    /// Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call. This account
    /// bears the admin role (for the granted role).
    /// Expected in cases where the role was granted using the internal
    /// [`AccessControl::grant_role`].
    #[allow(missing_docs)]
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    /// Emitted when `account` is revoked `role`.
    ///
    /// `sender` is the account that originated the contract call:
    ///   - if using `revoke_role`, it is the admin role bearer.
    ///   - if using `renounce_role`, it is the role bearer (i.e. `account`).
    #[allow(missing_docs)]
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
}

sol! {
    /// The `account` is missing a role.
    ///
    /// * `account` - Account that was found to not be authorized.
    /// * `needed_role` - The missing role.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error AccessControlUnauthorizedAccount(address account, bytes32 needed_role);
    /// The caller of a function is not the expected one.
    ///
    /// NOTE: Don't confuse with [`AccessControlUnauthorizedAccount`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    error AccessControlBadConfirmation();
}
sol! {
    /// Emitted when ownership gets transferred between accounts.
    ///
    /// * `previous_owner` - Address of the previous owner.
    /// * `new_owner` - Address of the new owner.
    #[allow(missing_docs)]
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
}



/// An error that occurred in the implementation of an [`AccessControl`]
/// contract.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// The caller account is missing a role.
    AccessUnauthorizedAccount(AccessControlUnauthorizedAccount),
    /// The caller of a afunction is not the expected one.
    BadConfirmation(AccessControlBadConfirmation),

}

sol_storage! {}


/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

#[public]
impl ATON {

 pub fn initialize_contract(&mut self, account: Address) -> Result<bool, ATONError> {
        self._grant_role(FixedBytes::from(constants::ARENATON_ENGINE_ROLE), account);
        Ok(true)
    }

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
            return Err(ATONError::ZeroEther(ZeroEther { sender }));
        }
        let _ = self._accumulate_commission(amount);
        // Mint equivalent ATON tokens to the sender
        let _ = self.mint(contract::address(), amount);

        // Emit the `DonateATON` event
        evm::log(DonateATON { sender, amount });
        Ok(true)
    }

    #[payable]
    pub fn deposit_eth(&mut self, _player: Address) -> Result<bool, Vec<u8>> {
        // self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
        let _ = self.mint(contract::address(), msg::value());
        Ok(true)
    }

    pub fn deposit_aton(&mut self, _player: Address, _amount: U256) -> Result<bool, Vec<u8>> {
        // let _ = self.access.only_role(constants::ARENATON_ENGINE_ROLE.into())?;
        let _ = self.transfer_from(_player, contract::address(), _amount);
        Ok(true)
    }

    pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
        if amount == U256::from(0) {
            return Err(ATONError::ZeroAton(ZeroAton {
                sender: msg::sender(),
            }));
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
    /// The default admin role. `[0; 32]` by default.

    /// Returns `true` if `account` has been granted `role`.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account to check for membership.
    #[must_use]
    pub fn has_role(&self, role: B256, account: Address) -> bool {
        self._roles.getter(role).has_role.get(account)
    }

    /// Checks if [`msg::sender`] has been granted `role`.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `role` - The role identifier.
    ///
    /// # Errors
    ///
    /// If [`msg::sender`] has not been granted `role`, then the error
    /// [`Error::AccessUnauthorizedAccount`] is returned.
    pub fn only_role(&self, role: B256) -> Result<(), Error> {
        self._check_role(role, msg::sender())
    }

    /// Returns the admin role that controls `role`. See [`Self::grant_role`]
    /// and [`Self::revoke_role`].
    ///
    /// To change a role's admin, use [`Self::_set_role_admin`].
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `role` - The role identifier.
    #[must_use]
    pub fn get_role_admin(&self, role: B256) -> B256 {
        *self._roles.getter(role).admin_role
    }

    /// Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a
    /// [`RoleGranted`] event.
    ///
    /// # Requirements:
    ///
    /// * The caller must have `role`'s admin role.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account which will be granted the role.
    ///
    /// # Errors
    ///
    /// If [`msg::sender`] has not been granted `role`, then the error
    /// [`Error::AccessUnauthorizedAccount`] is returned.
    ///
    /// # Events
    ///
    /// May emit a [`RoleGranted`] event.
    pub fn grant_role(&mut self, role: B256, account: Address) -> Result<(), Error> {
        let admin_role = self.get_role_admin(role);
        self.only_role(admin_role)?;
        self._grant_role(role, account);
        Ok(())
    }


    /// Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a [`RoleRevoked`] event.
    ///
    /// # Requirements:
    ///
    /// * The caller must have `role`'s admin role.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account which will be revoked the role.
    ///
    /// # Errors
    ///
    /// If [`msg::sender`] has not been granted `role`, then the error
    /// [`Error::AccessUnauthorizedAccount`] is returned.
    ///
    /// # Events
    ///
    /// May emit a [`RoleRevoked`] event.
    pub fn revoke_role(&mut self, role: B256, account: Address) -> Result<(), Error> {
        let admin_role = self.get_role_admin(role);
        self.only_role(admin_role)?;
        self._revoke_role(role, account);
        Ok(())
    }

    /// Revokes `role` from the calling account.
    ///
    /// Roles are often managed via [`Self::grant_role`] and
    /// [`Self::revoke_role`]: this function's purpose is to provide a mechanism
    /// for accounts to lose their privileges if they are compromised (such as
    /// when a trusted device is misplaced).
    ///
    /// # Requirements:
    ///
    /// * The caller must be `confirmation`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `confirmation` - The account which will be revoked the role.
    ///
    /// # Errors
    ///
    /// If [`msg::sender`] is not the `confirmation` address, then the error
    /// [`Error::BadConfirmation`] is returned.
    ///
    /// # Events
    ///
    /// If the calling account has its `role` revoked, emits a [`RoleRevoked`]
    /// event.
    pub fn renounce_role(&mut self, role: B256, confirmation: Address) -> Result<(), Error> {
        if msg::sender() != confirmation {
            return Err(Error::BadConfirmation(AccessControlBadConfirmation {}));
        }

        self._revoke_role(role, confirmation);
        Ok(())
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
    pub fn _player_commission(&mut self, player: Address) -> Result<U256, ATONError> {
        let pct_denom: U256 = U256::from(10000000);

        let _owed_per_token = self.accumulated_commission_per_token.get()
            - self.last_commission_per_token.get(player);
        let _unclaimed_commission = (self.balance_of(player) * _owed_per_token * pct_denom)
            / U256::from(10).pow(U256::from(18u8));
        Ok(_unclaimed_commission)
    }

    /// Sets `admin_role` as `role`'s admin role.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The identifier of the role we are changing the admin to.
    /// * `new_admin_role` - The new admin role.
    ///
    /// # Events
    ///
    /// Emits a [`RoleAdminChanged`] event.
    pub fn _set_role_admin(&mut self, role: B256, new_admin_role: B256) {
        let previous_admin_role = self.get_role_admin(role);
        self._roles.setter(role).admin_role.set(new_admin_role);
        evm::log(RoleAdminChanged {
            role,
            previous_admin_role,
            new_admin_role,
        });
    }

    /// Checks if `account` has been granted `role`.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account to check for membership.
    ///
    /// # Errors
    ///
    /// If [`msg::sender`] has not been granted `role`, then the error
    /// [`Error::AccessUnauthorizedAccount`] is returned.
    pub fn _check_role(&self, role: B256, account: Address) -> Result<(), Error> {
        if !self.has_role(role, account) {
            return Err(Error::AccessUnauthorizedAccount(
                AccessControlUnauthorizedAccount {
                    account,
                    needed_role: role,
                },
            ));
        }

        Ok(())
    }

    /// Attempts to grant `role` to `account` and returns a boolean indicating
    /// if `role` was granted.
    ///
    /// Internal function without access restriction.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account which will be granted the role.
    ///
    /// # Events
    ///
    /// May emit a [`RoleGranted`] event.
    pub fn _grant_role(&mut self, role: B256, account: Address) -> bool {
        if self.has_role(role, account) {
            false
        } else {
            self._roles.setter(role).has_role.insert(account, true);
            evm::log(RoleGranted {
                role,
                account,
                sender: msg::sender(),
            });
            true
        }
    }

    /// Attempts to revoke `role` from `account` and returns a boolean
    /// indicating if `role` was revoked.
    ///
    /// Internal function without access restriction.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account which will be granted the role.
    ///
    /// # Events
    ///
    /// May emit a [`RoleRevoked`] event.
    pub fn _revoke_role(&mut self, role: B256, account: Address) -> bool {
        if self.has_role(role, account) {
            self._roles.setter(role).has_role.insert(account, false);
            evm::log(RoleRevoked {
                role,
                account,
                sender: msg::sender(),
            });
            true
        } else {
            false
        }
    }
}
