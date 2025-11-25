// SPDX-License-Identifier: MIT
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "./Whitelist/WhitelistConsumer.sol";
import "./interfaces/IStakeVault.sol";
import "./interfaces/IDdexEmitter.sol";
import "./interfaces/IProverPublicOutputs.sol";
import "./interfaces/IDdexSequencer.sol";

pragma solidity ^0.8.24;

contract DdexSequencer is
    WhitelistConsumer,
    OwnableUpgradeable,
    UUPSUpgradeable,
    IDdexSequencer
{
    event NewBlobSubmitted(bytes commitment, bytes32 image_id);
    event QueueMoved();
    event WhitelistingStatusChanged(bool current_status);
    event BlobAssigned(bytes32 blob, address assignedValidator);
    event BlobExpired(bytes32 blobhash);
    event BlobLifetimeChanged(
        uint256 _previousLifetime,
        uint256 _currentLifetime
    );

    struct Blob {
        bytes32 nextBlob;
        bool submitted;
        address proposer;
        bytes32 blobId;
        bytes32 imageId;
        address assignedValidator;
        uint256 submissionBlock;
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

    bytes32 public nextBlobAssignment;

    uint256 public headProcessingStartBlock;
    uint256 public headProcessingTimeInBlocks;

    uint256 blobLifetime;

    uint256[46] __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        address dataProvidersWhitelist,
        address validatorsWhitelist,
        address stakeVaultAddress
    ) public initializer {
        _setWhitelistAddress(dataProvidersWhitelist, DATA_PROVIDERS_WHITELIST);
        _setWhitelistAddress(validatorsWhitelist, VALIDATORS_WHITELIST);
        stakeVault = IStakeVault(stakeVaultAddress);
        __Ownable_init(msg.sender);
        blobLifetime = 262144;
    }

    function setDdexEmitter(IDdexEmitter _ddexEmitter) public onlyOwner {
        ddexEmitter = _ddexEmitter;
    }

    function setBlobLifetime(uint256 _newLifetime) external onlyOwner {
        uint256 previousLifetime = blobLifetime;
        blobLifetime = _newLifetime;

        emit BlobLifetimeChanged(previousLifetime, blobLifetime);
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
    ) external {
        _submitNewBlob(_imageId, _commitment, _blobSha2, 0);
    }

    function submitNewBlob(
        bytes32 _imageId,
        bytes memory _commitment,
        bytes32 _blobSha2,
        uint256 blobIndex
    ) external {
        _submitNewBlob(_imageId, _commitment, _blobSha2, blobIndex);
    }

    function _submitNewBlob(
        bytes32 _imageId,
        bytes memory _commitment,
        bytes32 _blobSha2,
        uint256 blobIndex
    ) internal _isWhitelistedOn(DATA_PROVIDERS_WHITELIST) {
        require(_imageId != bytes32(0), "DdexSequencer: ImageId cannot be 0");

        (bytes32 currentImageId, bytes32 previousImageId) = ddexEmitter
            .getSupportedBlobImageIds();

        require(
            currentImageId == _imageId || previousImageId == _imageId,
            "DdexSequencer: Unsupported imageId"
        );

        bytes32 newBlobhash;
        assembly {
            newBlobhash := blobhash(blobIndex)
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
        blobs[newBlobhash].submissionBlock = block.number;
        blobs[newBlobhash].imageId = _imageId;

        if (nextBlobAssignment == bytes32(0)) {
            nextBlobAssignment = newBlobhash;
        }

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
        require(
            isQueueHeadExpired() == false,
            "DdexSequencer: Head processing time expired"
        );
        require(
            msg.sender == blobs[blobQueueHead].assignedValidator,
            "DdexSequencer: msg.sender must be assignmened to head blob"
        );

        ddexEmitter.verifyAndEmit(_imageId, _journal, _seal, _cid);

        _moveQueue();
    }

    function assignBlob() external _isWhitelistedOn(VALIDATORS_WHITELIST) {
        require(blobQueueHead != bytes32(0), "DdexSequencer: Queue is empty");

        if (blobQueueHead == nextBlobAssignment) {
            blobs[nextBlobAssignment].assignedValidator = msg.sender;
            headProcessingStartBlock = block.number;
            emit BlobAssigned(nextBlobAssignment, msg.sender);
            nextBlobAssignment = blobs[nextBlobAssignment].nextBlob;
        } else if (isQueueHeadExpired() == true) {
            // TODO! Slash previous blobs[blobQueueHead].assignedValidator here!
            blobs[blobQueueHead].assignedValidator = msg.sender;
            headProcessingStartBlock = block.number;
            emit BlobAssigned(blobQueueHead, msg.sender);
        } else {
            require(
                nextBlobAssignment != bytes32(0),
                "DdexSequencer: All blobs assigned"
            );
            blobs[nextBlobAssignment].assignedValidator = msg.sender;
            emit BlobAssigned(nextBlobAssignment, msg.sender);
            nextBlobAssignment = blobs[nextBlobAssignment].nextBlob;
        }
    }

    function isQueueHeadExpired() public view returns (bool) {
        if (
            block.number > headProcessingStartBlock + headProcessingTimeInBlocks
        ) {
            return true;
        } else {
            return false;
        }
    }

    function getQueueHeadDetails() public view returns (Blob memory, bytes32) {
        return (blobs[blobQueueHead], blobQueueHead);
    }

    function setHeadProcessingTimeInBlocks(
        uint256 newTimeInBlocks
    ) public onlyOwner {
        require(
            newTimeInBlocks > 1,
            "DdexSequencer: Head processing time must be greater than 1"
        );
        headProcessingTimeInBlocks = newTimeInBlocks;
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
            headProcessingStartBlock = block.number;
        }
        emit QueueMoved();
    }

    function moveQueue() external onlyOwner {
        if (blobQueueHead == nextBlobAssignment) {
            nextBlobAssignment = blobs[blobQueueHead].nextBlob;
        }
        _moveQueue();
    }

    function removeExpiredBlob() external {
        require(
            blobs[blobQueueHead].submissionBlock + blobLifetime < block.number,
            "DdexSequencer: Blob is still considered alive"
        );
        bytes32 expiredBlob = blobQueueHead;
        _moveQueue();
        emit BlobExpired(expiredBlob);
    }

    function _deleteBlobQueueHead() private {
        blobs[blobQueueHead].submitted = false;
        blobs[blobQueueHead].nextBlob = bytes32(0);
        blobs[blobQueueHead].proposer = address(0);
        blobs[blobQueueHead].assignedValidator = address(0);
        blobs[blobQueueHead].imageId = bytes32(0);
        blobs[blobQueueHead].blobId = bytes32(0);
        blobs[blobQueueHead].submissionBlock = 0;
    }

    function _authorizeUpgrade(address) internal override onlyOwner {}
}
