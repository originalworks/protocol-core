// SPDX-License-Identifier: MIT
import "../DdexSequencer.sol";

pragma solidity ^0.8.24;

contract BrokenDdexSequencer is DdexSequencer {
    function submitBrokenBlob(
        bytes32 _imageId,
        bytes memory _commitment,
        bytes32 _blobSha2,
        bytes32 _blobhash
    ) public _isWhitelistedOn(DATA_PROVIDERS_WHITELIST) {
        require(_imageId != bytes32(0), "DdexSequencer: ImageId cannot be 0");

        (bytes32 currentImageId, bytes32 previousImageId) = ddexEmitter
            .getSupportedBlobImageIds();

        require(
            currentImageId == _imageId || previousImageId == _imageId,
            "DdexSequencer: Unsupported imageId"
        );

        bytes32 blobId = sha256(abi.encodePacked(_blobhash, _blobSha2));
        require(
            blobs[_blobhash].submitted == false,
            "DdexSequencer: Blob already submitted"
        );
        blobs[_blobhash].submitted = true;
        blobs[_blobhash].proposer = msg.sender;
        blobs[_blobhash].blobId = blobId;
        blobs[_blobhash].submissionBlock = block.number;
        blobs[_blobhash].imageId = _imageId;

        if (nextBlobAssignment == bytes32(0)) {
            nextBlobAssignment = _blobhash;
        }

        if (blobQueueHead == bytes32(0)) {
            blobQueueHead = _blobhash;
            blobQueueTail = _blobhash;
        } else {
            blobs[blobQueueTail].nextBlob = _blobhash;
            blobQueueTail = _blobhash;
        }
        emit NewBlobSubmitted(_commitment, _imageId);
    }
}
