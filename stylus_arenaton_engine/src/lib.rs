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
//   mapping(bytes8 => Event) private events;
//   mapping(address => U256)  players;

  // Array for tracking active events
  bytes8[]  activeEvents;
  bytes8[]  closedEvents;
    }

    // pub struct Event {
    //     active: bool,
    //     closed: bool,
    //     paid: bool,
    //     startDate: u64,
    //     sport: u8,
    //     winner: i8,
    //     total: [u256; 2],
    //     players: Vec<Address>,
    //     stakes: Vec<Stake>,
    // }   

    // pub struct Player {
    //     address: Address,
    //     team: u8,
    //     stakes: Vec<Stake>,
    // }

    // pub struct Stake {
    //     team: u8,
    //     amount_aton: U256,
    //     amount_eth: U256,
    // }
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
