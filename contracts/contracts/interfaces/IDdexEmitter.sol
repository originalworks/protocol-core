// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import "./IProverPublicOutputs.sol";

interface IDdexEmitter {
    event BlobProcessed(ProverPublicOutputs proverPublicOutputs);
    event BlobRejected(ProverPublicOutputs proverPublicOutputs);
    event ImageIdChanged(
        bytes1 target,
        bytes32 previousImageId,
        bytes32 currentImageId
    );

    function getSupportedBlobImageIds()
        external
        view
        returns (bytes32, bytes32);

    function getSupportedVerifierImageIds()
        external
        view
        returns (bytes32, bytes32);

    function verifyAndEmit(
        bytes32 _imageId,
        bytes memory _journal,
        bytes calldata _seal
    ) external returns (bytes32 blobDigest);
}
