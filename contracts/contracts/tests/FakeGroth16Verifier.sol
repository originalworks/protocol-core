// SPDX-License-Identifier: MIT
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../interfaces/IRiscZeroVerifier.sol";

pragma solidity ^0.8.24;

contract FakeGroth16Verifier is IRiscZeroVerifier {
    function verify(
        bytes calldata seal,
        bytes32 imageId,
        bytes32 journalDigest
    ) public view {
        require(true == true, "");
    }
}
