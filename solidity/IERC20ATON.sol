// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

interface IERC20ATON {
    // Standard ERC20 functions inherited from IERC20
    function totalSupply() external view returns (uint256);

    function balanceOf(address account) external view returns (uint256);

    function transfer(address to, uint256 amount) external returns (bool);

    function allowance(address owner, address spender) external view returns (uint256);

    function approve(address spender, uint256 amount) external returns (bool);

    function transferFrom(address from, address to, uint256 amount) external returns (bool);

    // EIP-2612 permit function
    /**
     * @dev Sets `value` as the allowance of `spender` over `owner`'s tokens,
     * given `owner`'s signed approval.
     *
     * Emits an {Approval} event.
     *
     * Requirements:
     * - `spender` cannot be the zero address.
     * - `deadline` must be a timestamp in the future.
     * - `v`, `r`, and `s` must be a valid `secp256k1` signature from `owner`
     *   over the EIP712-formatted function arguments.
     * - the signature must use `owner`'s current nonce (see {nonces}).
     */
    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external;

    // Additional ATON-specific functions
    function swap(uint256 amountAton) external returns (bool success);

    function donateATON() external payable;

    function stakeETH(address player, uint256 amount) external payable;

    function stakeATON(address player, uint256 amount) external;

    // Events
    event Swap(address indexed sender, uint256 amountAton);

    event ATONDonated(address indexed donor, uint256 amount);

    event Accumulate(
        uint256 newCommissionATON,
        uint256 accumulatedCommissionPerToken,
        uint256 totalCommissionInATON
    );

    // Optional: Include the nonces mapping and DOMAIN_SEPARATOR if needed
    function nonces(address owner) external view returns (uint256);

    function DOMAIN_SEPARATOR() external view returns (bytes32);
}
