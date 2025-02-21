// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "../lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import "./interfaces/IDdexEmitter.sol";
import {ImageID} from "./ImageID.sol";

contract DdexEmitter is
    Initializable,
    OwnableUpgradeable,
    IDdexEmitter,
    UUPSUpgradeable
{
    IRiscZeroVerifier riscZeroGroth16Verifier;
    address ddexSequencerAddress;
    bytes32 public imageId;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        IRiscZeroVerifier _riscZeroGroth16Verifier,
        address _ddexSequencerAddress
    ) public initializer {
        imageId = ImageID.DDEX_GUEST_ID;
        riscZeroGroth16Verifier = _riscZeroGroth16Verifier;
        ddexSequencerAddress = _ddexSequencerAddress;
        __Ownable_init(msg.sender);
    }

    function setImageId(bytes32 newImage) public onlyOwner {
        imageId = newImage;
    }

    function verifyAndEmit(
        bytes memory journal,
        bytes calldata seal
    ) external returns (bytes32 blobSha2) {
        require(
            msg.sender == ddexSequencerAddress,
            "msg.sender is not DdexSequencer"
        );

        ProverPublicOutputs memory proverPublicOutputs = abi.decode(
            journal,
            (ProverPublicOutputs)
        );
        riscZeroGroth16Verifier.verify(seal, imageId, sha256(journal));

        if (proverPublicOutputs.valid) {
            emit BlobProcessed(proverPublicOutputs);
        } else {
            emit BlobRejected(proverPublicOutputs);
        }

        return proverPublicOutputs.digest;
    }

    function _authorizeUpgrade(address) internal override onlyOwner {}
}
