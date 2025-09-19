// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "../Whitelist/WhitelistConsumer.sol";
import "../interfaces/IDdexSequencer.sol";

contract SmartAccount is WhitelistConsumer, UUPSUpgradeable {
    struct SubmitNewBlobInput {
        bytes32 imageId;
        bytes commitment;
        bytes32 blobSha2;
    }

    bytes1 public constant BLOB_SUBMITTERS_WHITELIST = 0x01;
    IDdexSequencer ddexSequencer;
    uint256[50] __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        address smartAccountWhitelist,
        address ddexSequencerAddress
    ) public initializer {
        _setWhitelistAddress(smartAccountWhitelist, BLOB_SUBMITTERS_WHITELIST);
        ddexSequencer = IDdexSequencer(ddexSequencerAddress);
    }

    function submitNewBlobBatch(
        SubmitNewBlobInput[] calldata inputs
    ) public isWhitelistedOn(BLOB_SUBMITTERS_WHITELIST) {
        require(
            msg.sender == address(this) ||
                IWhitelist(whitelists[BLOB_SUBMITTERS_WHITELIST]).isWhitelisted(
                    msg.sender
                ),
            "msg.sender not authorized"
        );

        for (uint i = 0; i < inputs.length; i++) {
            ddexSequencer.submitNewBlob(
                inputs[i].imageId,
                inputs[i].commitment,
                inputs[i].blobSha2,
                i
            );
        }
    }

    function _authorizeUpgrade(address) internal view override {
        require(msg.sender == address(this));
    }
}
