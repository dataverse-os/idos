// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseClient} from "../../contracts/client/DataverseClient.sol";
import {IDataverseRelayer} from "../../contracts/relayer/IDataverseRelayer.sol";

contract DataverseClientMock is DataverseClient {
    uint256 public offChainValue;

    uint256 public constant PAYMENT = 10 ether;

    constructor(address dataverseToken, address dataverseRelayer) DataverseClient(dataverseToken, dataverseRelayer) {}

    function requestOffChainValue(bytes32 imageId, string memory payload) external {
        IDataverseRelayer.RequestParams memory requestParams =
            _buildRequest(imageId, payload, address(this), this.fulfillOffChainValue.selector);
        _sendRequest(requestParams, PAYMENT);
    }

    function fulfillOffChainValue(bytes32 requestId, bytes memory responseData) external callback(requestId) {
        uint256 value = abi.decode(responseData, (uint256));
        offChainValue = value;
    }

    function cancelRequestOffChainValue(
        bytes32 requestId,
        uint256 payment,
        uint256 expiration,
        IDataverseRelayer.RequestParams memory requestParams
    ) external {
        _cancelRequest(requestId, payment, expiration, requestParams);
    }
}
