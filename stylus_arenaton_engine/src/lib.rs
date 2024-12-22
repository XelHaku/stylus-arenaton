#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

mod ownable;
use crate::ownable::Ownable;

mod control;
use crate::control::AccessControl;

mod constants;
mod structs;
use alloy_sol_types::sol;

// --- Use standard String ---
use std::string::String;

use alloy_primitives::{Address, U256, B256};
use stylus_sdk::{
    call::transfer_eth,
    contract,
    evm,
    msg,
    stylus_proc::{ public, sol_storage, SolidityError },
};
use stylus_sdk::prelude::*;
use alloy_primitives::FixedBytes;

/// Additional events and errors
sol! {
    event DonateATON(address indexed sender, uint256 amount);
    event Accumulate(uint256 new_commission, uint256 accumulated, uint256 total);
    error ZeroEther(address sender);
    error ZeroAton(address sender);
    error AlreadyInitialized();
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    ZeroEther(ZeroEther),
    ZeroAton(ZeroAton),
    AlreadyInitialized(AlreadyInitialized),
}

// `ArenatonEngine` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ArenatonEngine {
        #[borrow]
        Ownable ownable;
        #[borrow]
        AccessControl control;
//   uint256 private premium = 200000;
//   uint256 constant pct_denom = 10000000;

  // Mapping for storing event and player data
  mapping(bytes8 => Event)  events;
  mapping(address => Player)  players;

  // Array for tracking active events
  bytes8[]  activeEvents;
  bytes8[]  closedEvents;
    }

  

 /**
   * @dev Structure representing a player's data within the platform.
   * This structure includes details about the player's activity, level, and commission earnings.
   */
  struct Player {
    bytes8[] activeEvents; // Array of event IDs in which the player is currently participating.
    bytes8[] closedEvents; // Array of event IDs for events that the player participated in and that are now closed.
    uint32 level; // The player's current level, representing their experience or skill within the platform.
    uint256 claimedCommissionsByPlayer; // Total amount of commissions claimed by the player.
    uint256 lastCommissionPerTokenForPlayer; // The last recorded commission per token for the player, used to calculate unclaimed commissions.
  }

      /**
   * @dev Structure representing a player's stake in an event.
   * This structure holds the amount staked and the team the player has bet on.
   */
  pub struct Stake {
    uint256 amount; // The total amount of tokens staked by the player.
    uint8 team; // The team the player is betting on: 1 for Team A, 2 for Team B.
  }

  /**
   * @dev Structure representing an event for betting.
   * This structure includes all necessary details for managing the event, including stakes, players, and the event's status.
   */
  pub struct Event {
    bytes8 eventIdBytes; // Unique identifier for the event in bytes8 format.
    uint256 startDate; // The start date and time of the event.
    address[] players; // List of players who have placed stakes in the event.
    mapping(address => Stake) stakes; // Mapping of player addresses to their respective stakes.
    mapping(address => bool) stakeFinalized; // Mapping to track whether a player's stake has been finalized and paid out.
    uint256[2] total; // Total stakes for each team: index 0 for Team A, index 1 for Team B.
    int8 winner; // The winner of the event: 1 for Team A, 2 for Team B, -2 for a tie, -1 for no result yet, -3 for event canceled.
    uint8 sport; // Identifier representing the sport associated with the event.
    uint256 playersPaid; // Number of players who have been paid out.
    bool active; // Indicates whether the event is currently open for participation.
    bool closed; // Indicates whether the event has ended.
    bool paid; // Indicates whether all payouts for the event have been processed.
  }


}

// Remove or provide Erc20 trait below if needed
#[public]
#[inherit(Ownable, AccessControl)]
impl ArenatonEngine {
    /// Add a new event
    pub fn add_event(&mut self, event_id: String, start_date: U256, sport: u8) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }

    /// Stake with ATON
    pub fn stake_aton(&mut self, _event_id: String, _amount_aton: U256, _team: u8) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }

    /// Stake with ETH
    pub fn stake_eth(&mut self, _event_id: String, _team: u8) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }

       pub fn close_event(&mut self, _event_id: String, _winner: u8) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }

       pub fn pay_event(&mut self, _event_id: String, _batch_size: u128) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }

    pub fn get_event(&self, _event_id: String) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }

    pub fn get_event_list(&self) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }


    pub fn get_player_event_list(&self, _player: Address) -> Result<bool, ATONError> {
        // Your logic
        Ok(true)
    }
}

impl ArenatonEngine {
    // Additional private or internal functions
}

