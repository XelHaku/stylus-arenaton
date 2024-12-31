// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
mod erc20;
use crate::erc20::Erc20;

// Modules and imports
mod constants;

// use alloy_sol_types::sol;
use stylus_sdk::{alloy_sol_types::sol,call::transfer_eth, contract, evm, msg,alloy_primitives::{Address, B256, U256}};

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
        mapping(address => PlayerInfo) players;


        address owner;
        mapping(bytes32 => RoleData) _roles;

    }


    pub struct RoleData {
        /// Whether an account is member of a certain role.
        mapping(address => bool) has_role;
        /// The admin role for this role.
        bytes32 admin_role;
    }

    pub struct PlayerInfo {
        uint256 last_commission_per_token;
        uint256 claimed_commissions;
}
}

sol! {


    // ATON
    event CommissionAccumulate(uint256 indexed amount, uint256 indexed newAccPerToken, uint256 indexed totalCommission);

    error ZeroEther(address sender);
    error ZeroAton(address sender);
    error AlreadyInitialized();

        // Access Control
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);


    // Ownable
    error UnauthorizedAccount(address account);
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
    AlreadyInitialized(AlreadyInitialized),

    // Access Control
    UnauthorizedAccount(UnauthorizedAccount),
}

#[public]
#[inherit(Erc20)]
impl ATON {
    pub fn initialize(&mut self) -> Result<bool, ATONError> {
        if self.initialized.get() {
            // Access the value using .get()
            return Err(ATONError::AlreadyInitialized(AlreadyInitialized {})); // Add the error struct
        }
        self.initialized.set(true); // Set initialized to true
        self.owner.set(msg::sender());
        self._grant_role(constants::DEFAULT_ADMIN_ROLE.into(), msg::sender());
        Ok(true)
    }

    /// 4. Emit a `DonateATON` event.
    ///
    /// # Errors
    // #[payable]
    //     pub fn donate_eth(&mut self) -> Result<bool, ATONError> {
    //         let amount = msg::value(); // Ether sent with the transaction
    //         let sender = msg::sender(); // Address of the sender

    //         // Ensure the transaction includes some Ether to donate
    //         if amount == U256::from(0) {
    //             return Err(ATONError::ZeroEther(ZeroEther { sender }));
    //         }
    //         let _ = self.add_commission(amount);
    //         // Mint equivalent ATON tokens to the sender
    //         let _ = self.erc20.mint(contract::address(), amount);

    //         // Emit the `DonateATON` event
    //         evm::log(DonateATON { sender, amount });
    //         Ok(true)
    //     }

    #[payable]
    pub fn mint_aton(&mut self) -> Result<bool, ATONError> {
        let is_engine = self._has_role(constants::ARENATON_ENGINE_ROLE.into(), msg::sender());

        if is_engine == false {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account: msg::sender(),
            }));
        }

        let _ = self.erc20.mint(msg::sender(), msg::value());

        Ok(true)
    }

    pub fn accumulate_aton(&mut self, amount: U256) -> Result<bool, ATONError> {
        // Ensure the transaction includes some Ether to donate
        if amount == U256::from(0) {
            return Err(ATONError::ZeroAton(ZeroAton {
                sender: msg::sender(),
            }));
        }
        let _ = self.transfer(contract::address(), amount);
        let _ = self.add_commission(amount);

        // Emit the `DonateATON` event
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
        self.erc20
            ._transfer(caller, to, amount)
            .map(|_| true)
            .map_err(|_| ATONError::ZeroAton(ZeroAton { sender: caller }))
    }

    pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
        if amount == U256::from(0) {
            return Err(ATONError::ZeroAton(ZeroAton {
                sender: msg::sender(),
            }));
        }

        if self.erc20.balance_of(msg::sender()) < amount {
            return Err(ATONError::ZeroAton(ZeroAton {
                sender: msg::sender(),
            }));
        }

        if contract::balance() < amount {
            return Err(ATONError::ZeroEther(ZeroEther {
                sender: msg::sender(),
            })); // error
        }

        let _ = transfer_eth(msg::sender(), amount);

        Ok(true)
    }

    // pub fn summary(&mut self, player: Address) -> Result<(U256, U256, U256), ATONError> {
    //     Ok((
    //         self._player_commission(player),
    //         self.players.get(player).claimed_commissions.get(),
    //         *self.total_commission_in_aton,
    //     ))
    // }

    // pub fn is_oracle(&self, account: Address) -> bool {
    //     self._has_role(constants::ARENATON_ORACLE_ROLE.into(), account)
    // }

    // Ownable

    // Access Control

    pub fn update_role(
        &mut self,
        account: Address,
        role_id: u8,
        grant: bool, // Boolean to specify grant or revoke
    ) -> Result<(), ATONError> {
        if !self._has_role(
            self._get_role_admin(constants::ARENATON_ENGINE_ROLE.into()),
            msg::sender(),
        ) {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account,
            }));
        }
        if grant {
            if role_id == 1 {
                self._grant_role(constants::ARENATON_ENGINE_ROLE.into(), account);
            } else if role_id == 2 {
                self._grant_role(constants::ARENATON_ORACLE_ROLE.into(), account);
            }
        } else {
            if role_id == 1 {
                self._revoke_role(constants::ARENATON_ENGINE_ROLE.into(), account);
            } else if role_id == 2 {
                self._revoke_role(constants::ARENATON_ORACLE_ROLE.into(), account);
            }
        }

        Ok(())
    }
}

// Private Functions
impl ATON {
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
        let total_supply_tokens = self.erc20.total_supply();

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

    pub fn _get_role_admin(&self, role: B256) -> B256 {
        *self._roles.getter(role).admin_role
    }
    /// Returns the unclaimed commission for a player
    pub fn _player_commission(&self, player: Address) -> U256 {
        // 1) Figure out how much is owed per token since last time
        let owed_per_token = self
            .accumulated_commission_per_token
            .saturating_sub(self.players.get(player).last_commission_per_token.get());

        // 2) Multiply that by player balance
        let balance = self.erc20.balance_of(player);
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

        let mut info = self.players.setter(player);

        if unclaimed > U256::ZERO {
            // If the contract itself is the "player," pay to owner
            let pay_to = if player == contract::address() {
                self.owner.get()
            } else {
                player
            };

            // Transfer from contract to pay_to
            // Make sure the contract has enough balanceOf(contract).
            if self.erc20.balance_of(contract::address()) >= unclaimed {
                match self.erc20._transfer(contract::address(), pay_to, unclaimed) {
                    Ok(_) => {
                        let _claimed = info.claimed_commissions.get();
                        // Update claimed commissions for the real player in storage
                        info.claimed_commissions.set(unclaimed + _claimed);
                    }
                    Err(_) => {}
                }
            }
        }

        info.last_commission_per_token
            .set(self.accumulated_commission_per_token.get());
    }
    // Access Control

    pub fn _check_role(&self, role: B256, account: Address) -> Result<(), ATONError> {
        if !self._has_role(role, account) {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account,
            }));
        }

        Ok(())
    }

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
