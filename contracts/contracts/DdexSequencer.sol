// SPDX-License-Identifier: MIT
import "@openzeppelin/contracts/access/Ownable.sol";
import "./Whitelist/WhitelistConsumer.sol";
import "./interfaces/IStakeVault.sol";
import "./interfaces/IVerifier.sol";

pragma solidity ^0.8.24;

contract DdexSequencer is WhitelistConsumer, Ownable {
    event NewBlobSubmitted(bytes commitment);
    event MessageDigested(DdexMessageData data);

    struct Blob {
        bytes32 nextBlob;
        bool submitted;
        address proposer;
    }

    struct DdexMessageData {
        uint256 x;
    }

    bytes1 public constant DATA_PROVIDERS_WHITELIST = 0x01;
    bytes1 public constant VALIDATORS_WHITELIST = 0x02;

    bytes32 public blobQueueHead;
    bytes32 public blobQueueTail;

    IStakeVault stakeVault;
    IVerifier verifier;

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

    function setVerifier(IVerifier _verifier) public onlyOwner {
        verifier = _verifier;
    }

    function submitNewBlob(
        bytes memory commitment
    ) public isWhitelistedOn(DATA_PROVIDERS_WHITELIST) {
        bytes32 newBlobhash;
        assembly {
            newBlobhash := blobhash(0)
        }
        require(newBlobhash != bytes32(0), "Blob not found in tx");
        require(
            blobs[newBlobhash].submitted == false,
            "Blob already submitted"
        );
        blobs[newBlobhash].submitted = true;
        blobs[newBlobhash].proposer = msg.sender;

        if (blobQueueHead == bytes32(0)) {
            blobQueueHead = newBlobhash;
            blobQueueTail = newBlobhash;
        } else {
            blobs[blobQueueTail].nextBlob = newBlobhash;
            blobQueueTail = newBlobhash;
        }
        emit NewBlobSubmitted(commitment);
    }

    function submitProofOfProcessing(
        uint256 x, // TODO implement DdexMessageData[] type here with proper fields
        bytes calldata seal
    ) external isWhitelistedOn(VALIDATORS_WHITELIST) {
        require(blobQueueHead != bytes32(0), "Queue is empty");

        verifier.verify(x, seal);

        _moveQueue();
        emit MessageDigested(DdexMessageData(x));
    }

    function submitProofForFraudulentBlob(
        bool proof
    ) external isWhitelistedOn(VALIDATORS_WHITELIST) {
        require(blobQueueHead != bytes32(0), "Queue is empty");

        bool isValid = proof; // TODO: implement actual logic of checking the proof for the blobQueueHead

        require(isValid, "Invalid proof");

        stakeVault.slashStake(blobs[blobQueueHead].proposer);

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

    function _deleteBlobQueueHead() private {
        blobs[blobQueueHead].submitted = false;
        blobs[blobQueueHead].nextBlob = bytes32(0);
        blobs[blobQueueHead].proposer = address(0);
    }
}
