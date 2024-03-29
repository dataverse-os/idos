// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {IDataverseRelayer} from "../relayer/IDataverseRelayer.sol";
import {DataverseToken} from "../token/DataverseToken.sol";

interface IDataverseClient {
    enum RequestStatus {
        Disabled,
        Pending,
        Fulfilled
    }

    event RequestSent(bytes32 requestId);
    event RequestCanceled(bytes32 requestId);
    event RequestFulfilled(bytes32 requestId);

    error NotDataverseRelayer();

    /**
     * @notice Returns request status of given request ID.
     * @param requestId The given requst ID.
     * @return RequestStauts An enumeration containing request stauts.
     */
    function getRequestStatus(bytes32 requestId) external view returns (RequestStatus);
}
