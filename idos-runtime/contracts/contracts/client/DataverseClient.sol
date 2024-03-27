// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {IDataverseRelayer} from "../relayer/IDataverseRelayer.sol";
import {IDataverseClient} from "./IDataverseClient.sol";
import {DataverseToken} from "../token/DataverseToken.sol";

abstract contract DataverseClient is IDataverseClient, ERC165 {
    DataverseToken public immutable DATAVERSE_TOKEN;
    IDataverseRelayer public immutable DATAVERSE_RELAYER;

    address internal constant SENDER_SLOT = address(0);
    uint256 internal constant PAYMENT_SLOT = 0;

    uint256 public nonce;

    mapping(bytes32 => RequestStatus) internal _requestStatus;

    constructor(address dataverseToken, address dataverseRelayer) {
        DATAVERSE_TOKEN = DataverseToken(dataverseToken);
        DATAVERSE_RELAYER = IDataverseRelayer(dataverseRelayer);
    }

    /**
     * @notice Modifer to ensure caller is DataverseRelayer and update the request status of given request ID.
     * @param requestId The given request ID.
     */
    modifier callback(bytes32 requestId) {
        if (msg.sender != address(DATAVERSE_RELAYER)) {
            revert NotDataverseRelayer();
        }
        _requestStatus[requestId] = RequestStatus.Fulfilled;
        _;
        emit RequestFulfilled(requestId);
    }

    /**
     * @inheritdoc ERC165
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IDataverseClient).interfaceId || super.supportsInterface(interfaceId);
    }

    /**
     * @inheritdoc IDataverseClient
     */
    function getRequestStatus(bytes32 requestId) external view returns (RequestStatus) {
        return _requestStatus[requestId];
    }

    function _buildRequest(bytes32 imageId, string memory payload, address callbackAddr, bytes4 callbackFunc)
        internal
        view
        returns (IDataverseRelayer.RequestParams memory)
    {
        return IDataverseRelayer.RequestParams({
            imageId: imageId,
            payload: payload,
            callbackAddr: callbackAddr,
            callbackFunc: callbackFunc,
            nonce: nonce
        });
    }

    function _sendRequest(IDataverseRelayer.RequestParams memory requestParams, uint256 payment) internal {
        bytes32 requestId = _calcRequestId(address(this), requestParams);
        ++nonce;
        bytes memory encodedRequest =
            abi.encodeWithSelector(IDataverseRelayer.request.selector, SENDER_SLOT, PAYMENT_SLOT, requestParams);
        DATAVERSE_TOKEN.transferAndCall(address(DATAVERSE_RELAYER), payment, encodedRequest);

        _requestStatus[requestId] = RequestStatus.Pending;

        emit RequestSent(requestId);
    }

    function _cancelRequest(
        bytes32 requestId,
        uint256 payment,
        uint256 expiration,
        IDataverseRelayer.RequestParams memory requestParams
    ) internal {
        DATAVERSE_RELAYER.cancelRequest(requestId, payment, expiration, requestParams);
        _requestStatus[requestId] = RequestStatus.Disabled;
        emit RequestCanceled(requestId);
    }

    function _calcRequestId(address sender, IDataverseRelayer.RequestParams memory requestParams)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(sender, requestParams));
    }
}
