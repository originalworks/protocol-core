// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct ProvedMessage {
    string message_id;
}

struct ProverPublicOutputs {
    bool is_valid;
    bytes32 digest;
    ProvedMessage[] messages;
}
