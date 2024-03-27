// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {IDataverseRelayer} from "../relayer/IDataverseRelayer.sol";

interface IDataverseVerifier {
    error VerifyFailed();
    error NotAuthorizedSubmmiter();

    function isSubmitterAuthorized(address submitter) external returns (bool);

    function authorizeSubmitter(address submitter, bool value) external;

    function submit(
        bytes32 requestId,
        IDataverseRelayer.RequestParams memory requestParams,
        uint256 payment,
        uint256 expiration,
        bytes memory responseData
    ) external returns (bool);
}
