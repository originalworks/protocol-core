// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IStakeVault {
    function slashStake(address _address) external;
}
