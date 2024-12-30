// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
mod erc20;
use crate::erc20::Erc20;

// Modules and imports
mod constants;
mod structs;
use alloy_sol_types::sol;

use alloy_primitives::{Address, B256, U256};

use alloy_primitives::FixedBytes;
use stylus_sdk::{
    call::transfer_eth,
    contract, evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

use stylus_sdk::prelude::*;

// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ATON {
        bool initialized ;
        
        #[borrow]
        Erc20 erc20;

        uint256  accumulated_commission_per_token;
        uint256  total_commission_in_aton;
        uint256  current_pot;
        mapping(address => uint256) last_commission_per_token;
        mapping(address => uint256) claimed_commissions;


        address owner;
        mapping(bytes32 => RoleData) _roles;

    }


    pub struct RoleData {
        /// Whether an account is member of a certain role.
        mapping(address => bool) has_role;
        /// The admin role for this role.
        bytes32 admin_role;
    }
}

sol! {


    // ATON
    event DonateATON(address indexed sender, uint256 amount);
    event AccumulateATON(uint256 new_commission,uint256 total);
    error ZeroEther(address sender);
    error ZeroAton(address sender);
    error AlreadyInitialized();

        // Access Control
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previous_admin_role, bytes32 indexed new_admin_role);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    error AccessControlUnauthorizedAccount(address account, bytes32 needed_role);
    error AccessControlBadConfirmation();


    // Ownable
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    error OwnableUnauthorizedAccount(address account);
    error OwnableInvalidOwner(address owner);
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
    AlreadyInitialized(AlreadyInitialized),

    // Access Control
    AccessUnauthorizedAccount(AccessControlUnauthorizedAccount),
    BadConfirmation(AccessControlBadConfirmation),
    // Ownable
    UnauthorizedAccount(OwnableUnauthorizedAccount),
    InvalidOwner(OwnableInvalidOwner),
}

#[public]
#[inherit(Erc20)]
impl ATON {
    pub fn initialize_contract(&mut self) -> Result<bool, ATONError> {
        if self.initialized.get() {
            // Access the value using .get()
            return Err(ATONError::AlreadyInitialized(AlreadyInitialized {})); // Add the error struct
        }
        self.initialized.set(true); // Set initialized to true
        self.owner.set(msg::sender());
        self._grant_role(
            FixedBytes::from(constants::DEFAULT_ADMIN_ROLE),
            msg::sender(),
        );
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
        let _ = self.erc20.mint(contract::address(), amount);

        // Emit the `DonateATON` event
        evm::log(DonateATON { sender, amount });
        Ok(true)
    }

    pub fn accumulate_aton(&mut self, amount: U256) -> Result<bool, ATONError> {

        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
            return Err(ATONError::ZeroAton(ZeroAton { sender: msg::sender() }));
        }
        let _ = self._accumulate_commission(amount);
        let _ = self.erc20.transfer(contract::address(), amount);

        // Emit the `DonateATON` event
        evm::log(AccumulateATON {
            new_commission: amount,
            total: amount,
        });
        Ok(true)
    }

    #[payable]
    pub fn mint_aton_from_eth(&mut self) -> Result<bool, Vec<u8>> {
        self._check_role(constants::ARENATON_ENGINE_ROLE.into(), msg::sender())?;
        let _ = self.erc20.mint(msg::sender(), msg::value());

        Ok(true)
    }

    pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
        if amount == U256::from(0) {
            return Err(ATONError::ZeroAton(ZeroAton {
                sender: msg::sender(),
            }));
        }
        let balance_aton = self.erc20.balance_of(msg::sender());

        if balance_aton < amount {
            return Err(ATONError::ZeroAton(ZeroAton {
                sender: msg::sender(),
            }));
        }
        let balance_eth = contract::balance();

        if balance_eth < amount {
            return Ok(true); // error
        }

        let _ = transfer_eth(msg::sender(), amount); // these two are equivalent

        Ok(true)
    }

    pub fn summary(&mut self) -> Result<(U256, U256, U256), ATONError> {
        let player_commission = self._player_commission(msg::sender())?;

        let player_claimed = self.claimed_commissions.get(msg::sender());
        Ok((
            player_commission,
            *self.total_commission_in_aton,
            player_claimed,
        ))
    }

    pub fn is_oracle(&self, account: Address) -> bool {
        self._has_role(
            FixedBytes::from(constants::ARENATON_ORACLE_ROLE),
            account,
        )
    }

    pub fn is_engine(&self, account: Address) -> bool {
        self._has_role(
            FixedBytes::from(constants::ARENATON_ENGINE_ROLE),
            account,
        )
    }

    // Ownable

    // Access Control
 


    #[must_use]
    pub fn get_role_admin(&self, role: B256) -> B256 {
        *self._roles.getter(role).admin_role
    }

    pub fn grant_engine_and_oracle_role(
        &mut self,
        account: Address,
        role_id: u8,
    ) -> Result<(), ATONError> {
        let admin_role = self.get_role_admin(FixedBytes::from(constants::ARENATON_ENGINE_ROLE));
        self._check_role(admin_role , msg::sender())?;   
        if role_id == 1 {
            self._grant_role(FixedBytes::from(constants::ARENATON_ENGINE_ROLE), account);
            // Add missing closing parenthesis
        }
        if role_id == 2 {
            self._grant_role(FixedBytes::from(constants::ARENATON_ORACLE_ROLE), account);
            // Add missing closing parenthesis
        }
        Ok(())
    }

    pub fn revoke_engine_and_oracle_role(
        &mut self,
        account: Address,
        role_id: u8,
    ) -> Result<(), ATONError> {
        let admin_role = self.get_role_admin(FixedBytes::from(constants::ARENATON_ENGINE_ROLE));
        self._check_role(admin_role , msg::sender())?;   

        if role_id == 1 {
            self._revoke_role(FixedBytes::from(constants::ARENATON_ENGINE_ROLE), account);
        }
        if role_id == 2 {
            self._revoke_role(FixedBytes::from(constants::ARENATON_ORACLE_ROLE), account);
        }
        Ok(())
    }
}

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
            evm::log(AccumulateATON {
                new_commission: new_commission_aton,
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
        let _unclaimed_commission = (self.erc20.balance_of(player) * _owed_per_token * pct_denom)
            / U256::from(10).pow(U256::from(18u8));
        Ok(_unclaimed_commission)
    }

    // Access Control
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
    pub fn _check_role(&self, role: B256, account: Address) -> Result<(), ATONError> {
        if !self._has_role(role, account) {
            return Err(ATONError::AccessUnauthorizedAccount(
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
        if self._has_role(role, account) {
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
        if self._has_role(role, account) {
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

    pub fn _has_role(&self, role: B256, account: Address) -> bool {
        self._roles.getter(role).has_role.get(account)
    }
}
