// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "../lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import "./interfaces/IDdexEmitter.sol";

contract DdexEmitter is
    Initializable,
    OwnableUpgradeable,
    IDdexEmitter,
    UUPSUpgradeable
{
    IRiscZeroVerifier riscZeroGroth16Verifier;
    address ddexSequencerAddress;
    bytes32 imageId;

    bool public allesKlar;

    event DigestedBlobDetails(uint256 x);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        IRiscZeroVerifier _riscZeroGroth16Verifier,
        address _ddexSequencerAddress,
        bytes32 _imageId
    ) public initializer {
        riscZeroGroth16Verifier = _riscZeroGroth16Verifier;
        ddexSequencerAddress = _ddexSequencerAddress;
        imageId = _imageId;
        __Ownable_init(msg.sender);
    }

    function verifyAndEmit(uint256 x, bytes calldata seal) public {
        require(
            msg.sender == ddexSequencerAddress,
            "msg.sender is not DdexSequencer"
        );
        bytes memory journal = abi.encode(x);
        riscZeroGroth16Verifier.verify(seal, imageId, sha256(journal));

        emit DigestedBlobDetails(x);
    }

    function _authorizeUpgrade(address) internal override onlyOwner {}
}
