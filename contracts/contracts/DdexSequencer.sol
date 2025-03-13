// SPDX-License-Identifier: MIT
import "@openzeppelin/contracts/access/Ownable.sol";
import "./Whitelist/WhitelistConsumer.sol";
import "./interfaces/IStakeVault.sol";
import "./interfaces/IDdexEmitter.sol";
import "./interfaces/IProverPublicOutputs.sol";

pragma solidity ^0.8.24;

contract DdexSequencer is WhitelistConsumer, Ownable {
    event NewBlobSubmitted(bytes commitment, bytes32 image_id);
    event WhitelistingStatusChanged(bool current_status);

    struct Blob {
        bytes32 nextBlob;
        bool submitted;
        address proposer;
        bytes32 blobId;
        bytes32 imageId;
    }

    bytes1 public constant DATA_PROVIDERS_WHITELIST = 0x01;
    bytes1 public constant VALIDATORS_WHITELIST = 0x02;

    bytes32 public blobQueueHead;
    bytes32 public blobQueueTail;

    IStakeVault stakeVault;
    IDdexEmitter public ddexEmitter;

    // temporary solution for open alpha tests
    bool whitelistsDisabled;

    mapping(bytes32 => Blob) public blobs;

    constructor(
        address dataProvidersWhitelist,
        address validatorsWhitelist,
        address stakeVaultAddress
    ) Ownable(msg.sender) {
        _setWhitelistAddress(dataProvidersWhitelist, DATA_PROVIDERS_WHITELIST);
        _setWhitelistAddress(validatorsWhitelist, VALIDATORS_WHITELIST);
        stakeVault = IStakeVault(stakeVaultAddress);
    }

    function setDdexEmitter(IDdexEmitter _ddexEmitter) public onlyOwner {
        ddexEmitter = _ddexEmitter;
    }

    // temporary solution for open alpha tests
    function setWhitelistingStatus(bool _disabled) public onlyOwner {
        if (whitelistsDisabled != _disabled) {
            whitelistsDisabled = _disabled;
            emit WhitelistingStatusChanged(_disabled);
        }
    }

    modifier _isWhitelistedOn(bytes1 whitelistId) {
        require(
            whitelistsDisabled ||
                IWhitelist(whitelists[whitelistId]).isWhitelisted(msg.sender),
            "DdexSequencer: Sender is not whitelisted"
        );

        _;
    }

    function submitNewBlob(
        bytes32 _imageId,
        bytes memory _commitment,
        bytes32 _blobSha2
    ) public _isWhitelistedOn(DATA_PROVIDERS_WHITELIST) {
        require(_imageId != bytes32(0), "DdexSequencer: ImageId cannot be 0");

        (bytes32 currentImageId, bytes32 previousImageId) = ddexEmitter
            .getSupportedBlobImageIds();

        require(
            currentImageId == _imageId || previousImageId == _imageId,
            "DdexSequencer: Unsupported imageId"
        );

        bytes32 newBlobhash;
        assembly {
            newBlobhash := blobhash(0)
        }
        require(
            newBlobhash != bytes32(0),
            "DdexSequencer: Blob not found in tx"
        );

        bytes32 blobId = sha256(abi.encodePacked(newBlobhash, _blobSha2));
        require(
            blobs[newBlobhash].submitted == false,
            "DdexSequencer: Blob already submitted"
        );
        blobs[newBlobhash].submitted = true;
        blobs[newBlobhash].proposer = msg.sender;
        blobs[newBlobhash].blobId = blobId;
        blobs[newBlobhash].imageId = _imageId;

        if (blobQueueHead == bytes32(0)) {
            blobQueueHead = newBlobhash;
            blobQueueTail = newBlobhash;
        } else {
            blobs[blobQueueTail].nextBlob = newBlobhash;
            blobQueueTail = newBlobhash;
        }
        emit NewBlobSubmitted(_commitment, _imageId);
    }

    function submitProof(
        bytes32 _imageId,
        bytes memory _journal,
        bytes calldata _seal,
        string memory _cid
    ) external _isWhitelistedOn(VALIDATORS_WHITELIST) {
        require(blobQueueHead != bytes32(0), "DdexSequencer: Queue is empty");

        ddexEmitter.verifyAndEmit(_imageId, _journal, _seal, _cid);

        _moveQueue();
    }

    function _moveQueue() private {
        if (blobQueueHead == blobQueueTail) {
            _deleteBlobQueueHead();
            blobQueueHead = bytes32(0);
            blobQueueTail = bytes32(0);
        } else {
            bytes32 newBlobQueueHead = blobs[blobQueueHead].nextBlob;
            _deleteBlobQueueHead();
            blobQueueHead = newBlobQueueHead;
        }
    }

    function moveQueue() external onlyOwner {
        _moveQueue();
    }

    function _deleteBlobQueueHead() private {
        blobs[blobQueueHead].submitted = false;
        blobs[blobQueueHead].nextBlob = bytes32(0);
        blobs[blobQueueHead].proposer = address(0);
    }
}
