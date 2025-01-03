#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
mod constants;
mod control;
// mod ownable;
mod structs;
mod tools;

// use crate::ownable::Ownable;
use crate::tools::{bytes32_to_string, string_to_bytes32};

use crate::control::AccessControl;

use alloy_sol_types::sol;

// --- Use standard String ---
use alloy_primitives::FixedBytes;
use alloy_primitives::Uint;
use alloy_primitives::{Address, U256};
use std::string::String;
use stylus_sdk::prelude::*;
use stylus_sdk::{
    block,
    call::{call, Call},
    contract, evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

sol_interface! {
    interface IATON {
    function mintAton() external payable returns (bool);
        function isOracle(address account) external view returns (bool);

    function transferFrom(address from, address to, uint256 value) external returns (bool);

 function approve(address spender, uint256 value) external returns (bool);

    function allowance(address owner, address spender) external view returns (uint256);
}
}

sol! {
    event DonateATON(address indexed sender, uint256 amount);
    event Accumulate(uint256 new_commission, uint256 accumulated, uint256 total);
    error ZeroEther(address sender);
    error ZeroAton(address sender);
    error AlreadyInitialized();
    error AleadyAdded();
    error AlreadyStarted();
    error WrongStatus();
error NotAuthorized();
    event AddEvent(        bytes8 event_id,
        uint64 start_date,
        uint8 sport,
    );
        // Ownership
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
    AleadyAdded(AleadyAdded),
    AlreadyStarted(AlreadyStarted),
    NotAuthorized(NotAuthorized),
    WrongStatus(WrongStatus),
    UnauthorizedAccount(OwnableUnauthorizedAccount),
    InvalidOwner(OwnableInvalidOwner),
}
// `ArenatonEngine` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ArenatonEngine {

  // Mapping for storing event and player data
  mapping(bytes8 => Event)  events;
  mapping(address => Player)  players;

  // Array for tracking active events
  bytes8[]  active_events;
  bytes8[]  closed_events;

        bool initialized ;

 uint256 number;
 address aton_address;
 address oracle_address;
 address vault_address;

    }



 /**
   * @dev Structure representing a player's data within the platform.
   * This structure includes details about the player's activity, level, and commission earnings.
   */
  struct Player {
    bytes8[] active_events; // Array of event IDs in which the player is currently participating.
    bytes8[] closed_events; // Array of event IDs for events that the player participated in and that are now closed.
    uint32 level; // The player's current level, representing their experience or skill within the platform.
  }

      /**
   * @dev Structure representing a player's stake in an event.
   * This structure holds the amount staked and the team the player has bet on.
   */
  pub struct Stake {
    uint256 amount; // The total amount of tokens staked by the player.
    uint8 team; // The team the player is betting on: 1 for Team A, 2 for Team B.
uint64 timestamp;
}

  /**
   * @dev Structure representing an event for betting.
   * This structure includes all necessary details for managing the event, including stakes, players, and the event's status.
   */
  pub struct Event {
    bytes8 event_id_bytes; // Unique identifier for the event in bytes8 format.
    uint64 start_date; // The start date and time of the event.
    address[] players; // List of players who have placed stakes in the event.
    Stake[] stakes; // Array of stakes made by players in the event.
    mapping(address => uint256) stake_player; // Mapping of player addresses to their respective stakes.
    mapping(address =>uint8) team_player; // Mapping of player addresses to their respective stakes.
    mapping(address => bool) paid_player; // Mapping to track whether a player's stake has been finalized and paid out.
    uint256[2] total; // Total stakes for each team: index 0 for Team A, index 1 for Team B.
    uint8 winner; // The winner of the event: 1 for Team A, 2 for Team B, -2 for a tie, -1 for no result yet, -3 for event canceled.
    uint8 sport; // Identifier representing the sport associated with the event.
    uint256 players_paid; // Number of players who have been paid out.

    uint8 status;// 0 unopende | 1 opened | 2 closed | 3 paid
  }


}

// Remove or provide Erc20 trait below if needed
#[public]
impl ArenatonEngine {
    pub fn initialize_arenaton_engine(
        &mut self,
        _aton_address: Address,
        _vault_address: Address,
    ) -> Result<bool, ATONError> {
        if self.initialized.get() {
            // Access the value using .get()
            return Err(ATONError::AlreadyInitialized(AlreadyInitialized {})); // Add the error struct
        }
        self.initialized.set(true); // Set initialized to true
        self.aton_address.set(_aton_address);
        self.vault_address.set(_vault_address);
        Ok(true)
    }
    pub fn set_oracle(&mut self, _oracle_address: Address) -> bool {
        if self.oracle_address.get() != contract::address()
            || self.oracle_address.get() != Address::ZERO
        {
            return false;
        }
        self.oracle_address.set(_oracle_address);
        true
    }
    pub fn is_oracle(&mut self) -> bool {
        if self.oracle_address.get() != msg::sender() {
            return false;
        }
        true
    }
    pub fn add_event(
        &mut self,
        event_id: String,
        start_date: u64,
        sport: u8,
    ) -> Result<bool, ATONError> {
        // Convert the error returned by `is_oracle` to `ATONError`
        let is_oracle = self.is_oracle();

        if !is_oracle {
            return Err(ATONError::NotAuthorized(NotAuthorized {}));
        }

        // Convert event_id to 8 bytes
        let id8 = string_to_bytes32(&event_id);
        // 2) "Borrow" a mutable reference to the storage for `events[id8]`
        let mut e = self.events.setter(id8);

        if e.status.get() != Uint::<8, 1>::from(0u8) {
            return Err(ATONError::AleadyAdded(AleadyAdded {}));
        }

        if block::timestamp() < start_date {
            return Err(ATONError::AlreadyStarted(AlreadyStarted {}));
        }
        // 3) Set fields in storage
        e.event_id_bytes.set(id8);
        e.start_date.set(Uint::<64, 1>::from(start_date));
        e.sport.set(Uint::<8, 1>::from(sport));
        e.winner.set(Uint::<8, 1>::from(99u8));
        e.players_paid.set(U256::ZERO);
        e.status.set(Uint::<8, 1>::from(1u8));
        // e is a `StorageGuardMut<Event>`

        // Update the first element in the `total` array
        e.total
            .get_mut(0)
            .expect("Failed to get the first element")
            .set(U256::ZERO);

        // Update the second element in the `total` array
        e.total
            .get_mut(1)
            .expect("Failed to get the second element")
            .set(U256::ZERO);

        // 4) Push to active_events
        self.active_events.push(id8);

        // 5) Emit the AddEvent(...) log
        evm::log(AddEvent {
            event_id: id8,
            start_date,
            sport,
        });

        Ok(true)
    }

    #[payable]
    pub fn stake(
        &mut self,
        _event_id: String,
        _amount: U256,
        _team: u8,
    ) -> Result<bool, ATONError> {
        let _player = msg::sender();
        let _value = msg::value(); // Ether sent with the transaction

        // Parse the const &str as a local Address variable
        let aton_contract = IATON::new(self.aton_address.get());

        if _value > U256::from(0) {
            let config = Call::new_in(self).value(_value);
            let _ = match aton_contract.mint_aton(config) {
                Ok(_) => Ok(true),
                Err(e) => Err(false),
            };
        } else {
            let config = Call::new_in(self);

            let _ = match aton_contract.transfer_from(config, _player, contract::address(), _amount)
            {
                Ok(_) => Ok(true),
                Err(e) => Err(false),
            };
        }

        let _ = self._add_stake(_event_id, _amount, _team);
        // Your logic
        Ok(true)
    }

    pub fn close_event(&mut self, _event_id: String, _winner: u8) -> Result<bool, ATONError> {
        let event_id_bytes = string_to_bytes32(&_event_id);
        // 2) "Borrow" a mutable reference to the storage for `events[event_id_bytes]`
        let mut e = self.events.setter(event_id_bytes);

        if e.status.get() != Uint::<8, 1>::from(1u8) {
            return Err(ATONError::WrongStatus(WrongStatus {}));
        }
        // 3) Set fields in storage
        e.winner.set(Uint::<8, 1>::from(_winner));
        e.status.set(Uint::<8, 1>::from(2u8));

        Ok(true)
    }
}

impl ArenatonEngine {
    pub fn _add_stake(
        &mut self,
        _event_id: String,
        _amount: U256,
        _team: u8,
    ) -> Result<bool, ATONError> {
        // convert _event_id to bytes8
        let mut event_id_bytes = [0u8; 8];
        let bytes = _event_id.as_bytes();
        let copy_len = bytes.len().min(event_id_bytes.len());
        event_id_bytes[..copy_len].copy_from_slice(&bytes[..copy_len]);

        // Convert [u8; 8] to FixedBytes<8>
        let event_id_key = FixedBytes::<8>::from(event_id_bytes);

        // Insert into the events mapping
        let event = self.events.get(event_id_key);
        //    validate event exists
        if event.status.get() != Uint::<8, 1>::from(1u8) {
            return Err(ATONError::WrongStatus(WrongStatus {}));
        }
        //validate evnt hast started
        if Uint::<64, 1>::from(block::timestamp()) < event.start_date.get() {
            return Err(ATONError::AlreadyStarted(AlreadyStarted {}));
        }
        // 2) "Borrow" a mutable reference to the storage for `events[event_id_bytes]`
        let mut e = self.events.setter(event_id_key);

        let _player = msg::sender();

        let _previous_stake = e.stake_player.get(_player);
        if _previous_stake != U256::ZERO {
            e.stake_player.insert(_player, _amount);
            e.team_player.insert(_player, Uint::<8, 1>::from(_team));
        }

        // 3) Set fields in storage
        // e is a `StorageGuardMut<Event>`

        // }

        // Your logic
        Ok(true)
    }
}
