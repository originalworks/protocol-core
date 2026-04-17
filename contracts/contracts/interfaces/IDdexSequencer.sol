// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IDdexSequencer {
    function submitNewBlob(
        bytes32 _imageId,
        bytes memory _commitment,
        bytes32 _blobSha2
    ) external;

    function submitNewBlob(
        bytes32 _imageId,
        bytes memory _commitment,
        bytes32 _blobSha2,
        uint256 blobIndex
    ) external;
}
