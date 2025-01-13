// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./IProverPublicOutputs.sol";

interface IDdexEmitter {
    event BlobProcessed(ProverPublicOutputs proverPublicOutputs);

    function verifyAndEmit(bytes memory journal, bytes calldata seal) external;
}
