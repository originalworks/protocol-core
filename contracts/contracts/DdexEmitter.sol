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
    IRiscZeroVerifier riscZeroGroth16Verifier; // deprecated, using mapping instead
    address ddexSequencerAddress;

    bytes1 public constant BLOB_CURRENT_IMAGE_ID = 0x01;
    bytes1 public constant BLOB_PREVIOUS_IMAGE_ID = 0x02;
    bytes1 public constant VERIFIER_CURRENT_IMAGE_ID = 0x03;
    bytes1 public constant VERIFIER_PREVIOUS_IMAGE_ID = 0x04;

    mapping(bytes1 => bytes32) imageIds;
    mapping(bytes32 => address) public riscZeroGroth16Verifiers;

    uint256[49] __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _riscZeroGroth16Verifier,
        address _ddexSequencerAddress
    ) public initializer {
        imageIds[BLOB_CURRENT_IMAGE_ID] = ImageID.DDEX_GUEST_ID;
        imageIds[VERIFIER_CURRENT_IMAGE_ID] = ImageID.DDEX_GUEST_ID;
        riscZeroGroth16Verifiers[
            ImageID.DDEX_GUEST_ID
        ] = _riscZeroGroth16Verifier;
        ddexSequencerAddress = _ddexSequencerAddress;
        __Ownable_init(msg.sender);
    }

    function setImageIds(
        bytes1[] memory _targets,
        bytes32[] memory _newImageIds,
        address[] memory _riscZeroGroth16Verifiers
    ) public onlyOwner {
        require(
            _targets.length == _newImageIds.length &&
                _newImageIds.length == _riscZeroGroth16Verifiers.length,
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
            riscZeroGroth16Verifiers[newImageId] = _riscZeroGroth16Verifiers[i];

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
        bytes calldata _seal,
        string memory _cid
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
        IRiscZeroVerifier(riscZeroGroth16Verifiers[_imageId]).verify(
            _seal,
            _imageId,
            sha256(_journal)
        );

        if (proverPublicOutputs.valid) {
            emit BlobProcessed(proverPublicOutputs, _cid);
        } else {
            emit BlobRejected(proverPublicOutputs, _cid);
        }

        return proverPublicOutputs.digest;
    }

    function _authorizeUpgrade(address) internal override onlyOwner {}
}
