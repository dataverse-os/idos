// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

interface IDataverseRelayer {
    struct RequestParams {
        bytes32 imageId;
        string payload;
        address callbackAddr;
        bytes4 callbackFunc;
        uint256 nonce;
    }

    event RequestReceived(
        bytes32 indexed requestId,
        address indexed sender,
        uint256 payment,
        uint256 expiration,
        RequestParams requestParams
    );
    event CancelRequestReceived(bytes32 indexed requestId, address indexed sender);
    event ResponseSent(bytes32 indexed requestId, address submitter);

    error NotDataverseToken();
    error NotDataverseVerifier();
    error NotDataverseClient();
    error InvalidRequestSelector();
    error DelegateCallRequestFailed();
    error InvalidCallbackAddr();
    error RequestNotFound();
    error RequestParamsHashMismatch();
    error CancelRequestIdMismatch();
    error CancelRequestParamsHashMismatch();
    error RequestNotExpired();

    function setDataverseVerifier(address dataverseVerifier) external;

    function request(address sender, uint256 payment, RequestParams memory requestParams) external;

    function cancelRequest(bytes32 requestId, uint256 payment, uint256 expiration, RequestParams memory requestParams)
        external;

    function response(
        address submmiter,
        bytes32 requestId,
        RequestParams memory requestParams,
        uint256 payment,
        uint256 expiration,
        bytes memory responseData
    ) external returns (bool);
}
