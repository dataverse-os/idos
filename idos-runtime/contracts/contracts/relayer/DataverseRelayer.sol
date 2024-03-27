// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC677Receiver} from "../token/ERC677/IERC677Receiver.sol";
import {IDataverseClient} from "../client/IDataverseClient.sol";
import {IDataverseRelayer} from "./IDataverseRelayer.sol";

contract DataverseRelayer is IDataverseRelayer, IERC677Receiver, Ownable {
    address public dataverseToken;
    address public dataverseVerifier;
    uint256 public constant EXPIRATION_INTERVAL = 5 minutes;
    mapping(bytes32 => bytes32) internal _requestParamsHashById;

    constructor(address dataverseToken_) Ownable(msg.sender) {
        _setDataverseToken(dataverseToken_);
    }

    modifier onlyDataverseToken() {
        if (msg.sender != dataverseToken) {
            revert NotDataverseToken();
        }
        _;
    }

    modifier onlyDataverseVerifier() {
        if (msg.sender != dataverseVerifier) {
            revert NotDataverseVerifier();
        }
        _;
    }

    /**
     * @inheritdoc IDataverseRelayer
     */
    function setDataverseVerifier(address dataverseVerifier_) external onlyOwner {
        _setDataverseVerifier(dataverseVerifier_);
    }

    /**
     * @inheritdoc IERC677Receiver
     */
    function onTokenTransfer(address sender, uint256 payment, bytes memory requestData) external onlyDataverseToken {
        bytes4 oracleRequestFunc;
        assembly {
            oracleRequestFunc := mload(add(requestData, 32))
        }

        if (oracleRequestFunc != this.request.selector) {
            revert InvalidRequestSelector();
        }

        // add *sender* and *payment* to the slot of requestData
        assembly {
            mstore(add(requestData, 36), sender)
            mstore(add(requestData, 68), payment)
        }

        (bool success,) = address(this).delegatecall(requestData); // calls request with delegatecall, msg.sender is still dataverse token.
        if (!success) {
            revert DelegateCallRequestFailed();
        }
    }

    /**
     * @inheritdoc IDataverseRelayer
     */
    function request(address sender, uint256 payment, RequestParams memory requestParams) external onlyDataverseToken {
        if (requestParams.callbackAddr == dataverseToken) {
            revert InvalidCallbackAddr();
        }
        if (!IERC165(sender).supportsInterface(type(IDataverseClient).interfaceId)) {
            revert NotDataverseClient();
        }
        uint256 expiration = block.timestamp + EXPIRATION_INTERVAL;
        bytes32 requestId = _calcRequestId(sender, requestParams);
        bytes32 requestParamsHash = _calcRequestParamsHash(payment, expiration, requestParams);
        _requestParamsHashById[requestId] = requestParamsHash;

        emit RequestReceived(requestId, sender, payment, expiration, requestParams);
    }

    /**
     * @inheritdoc IDataverseRelayer
     */
    function cancelRequest(bytes32 requestId, uint256 payment, uint256 expiration, RequestParams memory requestParams)
        external
    {
        if (requestId != _calcRequestId(msg.sender, requestParams)) {
            revert CancelRequestIdMismatch();
        }
        if (_requestParamsHashById[requestId] != _calcRequestParamsHash(payment, expiration, requestParams)) {
            revert CancelRequestParamsHashMismatch();
        }
        if (expiration > block.timestamp) {
            revert RequestNotExpired();
        }

        delete _requestParamsHashById[requestId];

        IERC20(dataverseToken).transfer(msg.sender, payment);

        emit CancelRequestReceived(requestId, msg.sender);
    }

    /**
     * @inheritdoc IDataverseRelayer
     */
    function response(
        address submitter,
        bytes32 requestId,
        RequestParams memory requestParams,
        uint256 payment,
        uint256 expiration,
        bytes memory responseData
    ) external onlyDataverseVerifier returns (bool) {
        if (_requestParamsHashById[requestId] == bytes32(0)) {
            revert RequestNotFound();
        }
        bytes32 requestParamsHash = _calcRequestParamsHash(payment, expiration, requestParams);
        if (_requestParamsHashById[requestId] != requestParamsHash) {
            revert RequestParamsHashMismatch();
        }
        delete _requestParamsHashById[requestId];

        (bool success,) =
            requestParams.callbackAddr.call(abi.encodeWithSelector(requestParams.callbackFunc, requestId, responseData));

        if (success) {
            IERC20(dataverseToken).transfer(submitter, payment);
        }

        emit ResponseSent(requestId, submitter);

        return success;
    }

    function _calcRequestId(address sender, RequestParams memory requestParams) internal pure returns (bytes32) {
        return keccak256(abi.encode(sender, requestParams));
    }

    function _calcRequestParamsHash(uint256 payment, uint256 expiration, RequestParams memory requestParams)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(payment, expiration, requestParams));
    }

    function _setDataverseToken(address dataverseToken_) internal {
        dataverseToken = dataverseToken_;
    }

    function _setDataverseVerifier(address dataverseVerifier_) internal {
        dataverseVerifier = dataverseVerifier_;
    }
}
