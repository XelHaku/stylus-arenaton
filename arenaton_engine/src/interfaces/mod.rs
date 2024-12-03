/// The `IATON` interface represents the Solidity contract with specified functions and behaviors.
/// It uses `sol_interface!` for code generation and easier integration with Solidity contracts.
sol_interface! {
    interface IATON {
        /// Initializes the contract.
        function initializeContract() external returns (bool);

        /// Gets the name of the token.
        function name() external pure returns (string);

        /// Gets the symbol of the token.
        function symbol() external pure returns (string);

        /// Gets the decimals for the token.
        function decimals() external pure returns (uint8);

        /// Gets the total supply of tokens.
        function totalSupply() external view returns (uint256);

        /// Gets the balance of a specific address.
        /// @param owner The address to query.
        /// @return The token balance of the address.
        function balanceOf(address owner) external view returns (uint256);

        /// Transfers tokens to a specific address.
        /// @param to The recipient address.
        /// @param value The amount of tokens to transfer.
        /// @return A boolean indicating success.
        function transfer(address to, uint256 value) external returns (bool);

        /// Transfers tokens from one address to another.
        /// @param from The source address.
        /// @param to The recipient address.
        /// @param value The amount of tokens to transfer.
        /// @return A boolean indicating success.
        function transferFrom(address from, address to, uint256 value) external returns (bool);

        /// Approves an address to spend tokens on behalf of the caller.
        /// @param spender The address allowed to spend tokens.
        /// @param value The amount of tokens to approve.
        /// @return A boolean indicating success.
        function approve(address spender, uint256 value) external returns (bool);

        /// Gets the allowance of an address to spend tokens on behalf of another.
        /// @param owner The owner of the tokens.
        /// @param spender The spender address.
        /// @return The remaining allowance.
        function allowance(address owner, address spender) external view returns (uint256);

        /// Mints tokens for debugging purposes.
        /// @return A boolean indicating success.
        function debugMintAton() external payable returns (bool);

        /// Allows the caller to donate Ether to the contract.
        /// @return A boolean indicating success.
        function donateEth() external payable returns (bool);

        /// Stakes Ether for a specific player.
        /// @param _player The player address.
        /// @return A boolean indicating success.
        function stakeEth(address _player) external payable returns (bool);

        /// Stakes a specified amount of tokens for a specific player.
        /// @param _player The player address.
        /// @param _amount The amount of tokens to stake.
        /// @return A boolean indicating success.
        function stakeAton(address _player, uint256 _amount) external returns (bool);

        /// Swaps a specified amount of tokens.
        /// @param amount The amount to swap.
        /// @return A boolean indicating success.
        function swap(uint256 amount) external returns (bool);

        /// Checks if an account has a specific role.
        /// @param role The role identifier.
        /// @param account The account address.
        /// @return A boolean indicating if the account has the role.
        function hasRole(bytes32 role, address account) external view returns (bool);

        /// Grants a specific role to an account.
        /// @param role The role identifier.
        /// @param account The account address.
        function grantRole(bytes32 role, address account) external;

        /// Revokes a specific role from an account.
        /// @param role The role identifier.
        /// @param account The account address.
        function revokeRole(bytes32 role, address account) external;
    }
}
