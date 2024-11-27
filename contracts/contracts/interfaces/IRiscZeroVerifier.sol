// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

interface IRiscZeroVerifier {
    function verify(
        bytes calldata seal,
        bytes32 imageId,
        bytes32 journalDigest
    ) external view;
}
