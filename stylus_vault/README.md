# ATON Smart Contract

The **ATON** smart contract is a core component of the Arenaton decentralized ecosystem, built using the Arbitrum Stylus SDK. It functions as a token-based commission and staking mechanism, enabling seamless interaction between Ether (ETH) and the ATON token, supporting donations, swaps, and transparent commission distribution.

## Features

### Core Functionalities

- **Token Management**: Fully compatible ERC-20 implementation for the ATON token.
- **ETH Donations**: Convert ETH to ATON tokens through donations.
- **Commission Accumulation**: Accumulates platform commissions in ATON tokens.
- **Swap Functionality**: Allows users to swap ATON tokens for ETH and vice versa.
- **Role-Based Access**: Secure contract access using `Ownable` and `AccessControl`.
- **Transparent Player Rewards**: Tracks and distributes unclaimed player commissions.

### Smart Contract Details

- **ABI Compatibility**: Interoperable with Solidity and Rust environments.
- **Gas Efficiency**: Optimized for lower transaction costs on the Arbitrum blockchain.
- **Error Handling**: Custom errors for precise feedback.

## Contract Architecture

### Storage

- **ERC-20 Token Support**: Implements the standard token functions and events.
- **Commission Tracking**:
  - `accumulated_commission_per_token`: Tracks cumulative commission per token.
  - `total_commission_in_aton`: Total commission accumulated in ATON tokens.
  - Player-specific mappings for claimed and unclaimed commissions.
- **Access Control**:
  - Roles managed through `AccessControl` for secure operations.

### Events

- `DonateATON`: Logs ETH or ATON donations.
- `Accumulate`: Logs updates to the commission accumulation.

### Errors

- `ZeroEther`: Raised when a donation of zero ETH is attempted.
- `ZeroAton`: Raised when operations are attempted with zero ATON tokens.
- `AlreadyInitialized`: Prevents re-initialization of the contract.

## Getting Started

### Prerequisites

- **Rust Environment**: Ensure Rust and `cargo-stylus` are installed.
- **Arbitrum Stylus SDK**: Use the Stylus SDK for development and deployment.

### Compilation

Compile the contract with:

```bash
cargo build --release
