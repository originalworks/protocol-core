// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";

contract AssetRegistry is Ownable {
    address ddexSequencer;
    address assetFactory;
    address constant tokenizableAsset =
        0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF;

    mapping(bytes32 => address) public assets;

    constructor(
        address _ddexSequencer,
        address _assetFactory
    ) Ownable(msg.sender) {
        ddexSequencer = _ddexSequencer;
        assetFactory = _assetFactory;
    }

    function setDdexSequencerAddress(address _ddexSequencer) public onlyOwner {
        ddexSequencer = _ddexSequencer;
    }

    function setAssetFactoryAddress(address _assetFactory) public onlyOwner {
        assetFactory = _assetFactory;
    }

    function registerAsset(bytes32 assetId) external {
        require(
            msg.sender == ddexSequencer,
            "Only DdexSequencer can register new asset"
        );
        assets[assetId] = tokenizableAsset;
    }

    function canBeTokenized(bytes32 assetId) public view returns (bool) {
        return assets[assetId] == tokenizableAsset;
    }

    function setAsTokenized(bytes32 assetId, address assetAddress) external {
        require(
            msg.sender == assetFactory,
            "Only AssetFactory can register new asset"
        );
        assets[assetId] = assetAddress;
    }
}
