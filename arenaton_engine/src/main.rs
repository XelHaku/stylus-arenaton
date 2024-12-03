#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
   arenaton_engine_stylus::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
