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

    bytes1 public constant BLOB_CURRENT_IMAGE_ID = 0x01;
    bytes1 public constant BLOB_PREVIOUS_IMAGE_ID = 0x02;
    bytes1 public constant VERIFIER_CURRENT_IMAGE_ID = 0x03;
    bytes1 public constant VERIFIER_PREVIOUS_IMAGE_ID = 0x04;

    mapping(bytes1 => bytes32) imageIds;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        IRiscZeroVerifier _riscZeroGroth16Verifier,
        address _ddexSequencerAddress
    ) public initializer {
        imageIds[BLOB_CURRENT_IMAGE_ID] = ImageID.DDEX_GUEST_ID;
        imageIds[VERIFIER_CURRENT_IMAGE_ID] = ImageID.DDEX_GUEST_ID;
        riscZeroGroth16Verifier = _riscZeroGroth16Verifier;
        ddexSequencerAddress = _ddexSequencerAddress;
        __Ownable_init(msg.sender);
    }

    function setImageIds(
        bytes1[] memory _targets,
        bytes32[] memory _newImageIds
    ) public onlyOwner {
        require(
            _targets.length == _newImageIds.length,
            "DdexEmitter: Mismatched array lengths"
        );

        for (uint256 i = 0; i < _targets.length; i++) {
            bytes1 target = _targets[i];
            bytes32 newImageId = _newImageIds[i];

            require(
                target > bytes1(0) && target < 0x05,
                "DdexEmitter: Invalid target"
            );

            bytes32 previousImageId = imageIds[target];
            imageIds[target] = newImageId;

            emit ImageIdChanged(target, previousImageId, newImageId);
        }
    }

    function getSupportedBlobImageIds()
        external
        view
        returns (bytes32, bytes32)
    {
        return (
            imageIds[BLOB_CURRENT_IMAGE_ID],
            imageIds[BLOB_PREVIOUS_IMAGE_ID]
        );
    }

    function getSupportedVerifierImageIds()
        external
        view
        returns (bytes32, bytes32)
    {
        return (
            imageIds[VERIFIER_CURRENT_IMAGE_ID],
            imageIds[VERIFIER_PREVIOUS_IMAGE_ID]
        );
    }

    function verifyAndEmit(
        bytes32 _imageId,
        bytes memory _journal,
        bytes calldata _seal
    ) external returns (bytes32 blobSha2) {
        require(
            msg.sender == ddexSequencerAddress,
            "msg.sender is not DdexSequencer"
        );
        require(_imageId != bytes32(0), "DdexEmitter: ImageId cannot be 0");
        require(
            imageIds[VERIFIER_CURRENT_IMAGE_ID] == _imageId ||
                imageIds[VERIFIER_PREVIOUS_IMAGE_ID] == _imageId,
            "DdexEmitter: Unsupported imageId"
        );

        ProverPublicOutputs memory proverPublicOutputs = abi.decode(
            _journal,
            (ProverPublicOutputs)
        );
        riscZeroGroth16Verifier.verify(_seal, _imageId, sha256(_journal));

        if (proverPublicOutputs.valid) {
            emit BlobProcessed(proverPublicOutputs);
        } else {
            emit BlobRejected(proverPublicOutputs);
        }

        return proverPublicOutputs.digest;
    }

    function _authorizeUpgrade(address) internal override onlyOwner {}
}
