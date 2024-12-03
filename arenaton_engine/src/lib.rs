// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use std::str::FromStr;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{ msg,alloy_primitives::U256, prelude::* };
use alloy_primitives::Address;
use stylus_sdk::call::{ Call, call };

// Define some persistent storage using the Solidity ABI.
// `ArenatonEngine` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ArenatonEngine {
        uint256 number;
    }
}
sol_interface! {
    interface IATON {
        function initializeContract() external returns (bool);
        function name() external pure returns (string);
        function symbol() external pure returns (string);
        function decimals() external pure returns (uint8);
        function totalSupply() external view returns (uint256);
        function balanceOf(address owner) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function approve(address spender, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function debugMintAton() external payable returns (bool);
        function donateEth() external payable returns (bool);
        function depositEth(address _player) external payable returns (bool);
        function depositAton(address _player, uint256 _amount) external returns (bool);
        function swap(uint256 amount) external returns (bool);
        function hasRole(bytes32 role, address account) external view returns (bool);
        function grantRole(bytes32 role, address account) external;
        function revokeRole(bytes32 role, address account) external;
    }
}

/// Declare that `ArenatonEngine` is a contract with the following external methods.
#[public]
impl ArenatonEngine {
    pub fn name(&mut self) -> String {
        // Make name() take &mut self
        let aton_contract = IATON::new(
            Address::from_str("0xa6e41ffd769491a42a6e5ce453259b93983a22ef").unwrap()
        );
        let config = Call::new_in(self); // Pass self (now &mut self) to Call::new_in()

        match aton_contract.name(config) {
            Ok(name) => name.into(),
            Err(err) => panic!("Error calling name(): {:?}", err),
        }
    }

#[payable]
pub fn stake_eth(&mut self, _player: Address) -> bool {
    let aton_contract = IATON::new(
        Address::from_str("0xa6e41ffd769491a42a6e5ce453259b93983a22ef").unwrap()
    );
    let config = Call::new_in(self).value(msg::value()); // Set the value in the config

    match aton_contract.deposit_eth(config, _player) {
        Ok(_) => true,
        Err(err) => panic!("Error calling stakeEth(): {:?}", err),
    }
}
    //    #[payable]
    //     pub fn send_via_call(&mut self, to: Address) -> Result<(), Vec<u8>> {
    //         call(Call::new_in(self).value(msg::value()), to, &[])?;
    //         Ok(())
    //     }

    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }
}
// cargo stylus deploy --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
//
