// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

interface IERC20ATON {
    function totalSupply() external view returns (uint256);

    function balanceOf(address account) external view returns (uint256);

    function transfer(address recipient, uint256 amount) external returns (bool);

    function allowance(address owner, address spender) external view returns (uint256);

    function approve(address spender, uint256 amount) external returns (bool);

    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

    function decimals() external view returns (uint8);

    function mint(address account, uint256 amount) external;

    function burn(address account, uint256 amount) external;

    // New Methods
    function donateATON() external payable;

    function stakeETH(address player) external payable;

    function stakeATON(address player, uint256 amount) external;
}
