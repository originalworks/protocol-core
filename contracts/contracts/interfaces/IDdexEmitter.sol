// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IDdexEmitter {
    function verifyAndEmit(uint256 x, bytes calldata seal) external;
}
