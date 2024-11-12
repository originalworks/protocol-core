// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "hardhat/console.sol";
import "./Whitelist/WhitelistConsumer.sol";
import "./IStakeVault.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract StakeVault is IStakeVault, WhitelistConsumer, Ownable {
    bytes1 constant VALIDATORS_WHITELIST = 0x02;
    uint16 constant slashRateDenominator = 10000;

    address ddexSequencer;
    uint16 slashRate;
    IERC20 stakeToken;
    uint256 minStakeAmount;

    struct Validator {
        bool approved;
        bool active;
        uint256 stake;
    }

    mapping(address => Validator) validators;

    constructor(
        address stakeTokenAddress,
        uint16 _slashRate
    ) Ownable(msg.sender) {
        stakeToken = IERC20(stakeTokenAddress);
        slashRate = _slashRate;
    }

    function slashStake(address _address) public {
        require(msg.sender == ddexSequencer, "msg.sender is not ddexSequencer");
        uint256 slashedAmount = ((validators[_address].stake * slashRate) /
            slashRateDenominator);
        validators[_address].stake -= slashedAmount;

        if (validators[_address].stake < minStakeAmount) {
            validators[_address].active = false;
        }
        // what to do with slashed stake?
    }

    function setSlashRate(uint16 _slashRate) public onlyOwner {
        slashRate = _slashRate;
    }

    function approveValidator(address validatorAddress) external onlyOwner {
        require(
            validators[validatorAddress].approved == false,
            "Already approved"
        );
        validators[validatorAddress].approved = true;
    }

    function increaseStake(
        uint256 increaseAmount
    ) external isWhitelistedOn(VALIDATORS_WHITELIST) {
        require(
            validators[msg.sender].approved == true,
            "Validator not approved"
        );
        stakeToken.transferFrom(msg.sender, address(this), increaseAmount);

        validators[msg.sender].stake += increaseAmount;
        if (
            validators[msg.sender].stake >= minStakeAmount &&
            validators[msg.sender].active == false
        ) {
            validators[msg.sender].active = true;
        }
    }
}
