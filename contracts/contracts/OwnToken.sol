// SPDX-License-Identifier: MIT
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

pragma solidity ^0.8.24;

contract OwnToken is ERC20 {
    constructor() ERC20("Own Token", "OWN") {
        _mint(msg.sender, 1000000000);
    }
}
