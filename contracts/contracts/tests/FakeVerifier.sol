// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract FakeVerifier {
    function verify(
        bytes calldata seal,
        bytes32 imageId,
        bytes32 journalDigest
    ) public view {}
}
