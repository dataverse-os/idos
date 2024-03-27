// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IDataverseRelayer} from "../relayer/IDataverseRelayer.sol";
import {IDataverseVerifier} from "./IDataverseVerifier.sol";

contract DataverseVerifier is IDataverseVerifier, Ownable {
    IDataverseRelayer public immutable DATAVERSE_RELAYER;

    mapping(address => bool) public isSubmitterAuthorized;

    modifier onlyAuthorizedSubmitter() {
        if (!isSubmitterAuthorized[msg.sender]) {
            revert NotAuthorizedSubmmiter();
        }
        _;
    }

    constructor(address dataverseRelayer) Ownable(msg.sender) {
        DATAVERSE_RELAYER = IDataverseRelayer(dataverseRelayer);
    }

    /**
     * @inheritdoc IDataverseVerifier
     */
    function authorizeSubmitter(address submitter, bool value) external onlyOwner {
        isSubmitterAuthorized[submitter] = value;
    }

    /**
     * @inheritdoc IDataverseVerifier
     */
    function submit(
        bytes32 requestId,
        IDataverseRelayer.RequestParams memory requestParams,
        uint256 payment,
        uint256 expiration,
        bytes memory responseData
    ) external onlyAuthorizedSubmitter returns (bool) {
        _verify();
        return IDataverseRelayer(DATAVERSE_RELAYER).response(
            msg.sender, requestId, requestParams, payment, expiration, responseData
        );
    }

    function _verify() internal pure {
        if (false) {
            revert VerifyFailed();
        }
    }
}
