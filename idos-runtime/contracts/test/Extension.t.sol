// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseClientMock} from "./mocks/DataverseClientMock.sol";
import {IDataverseClient} from "../contracts/client/IDataverseClient.sol";
import {IDataverseRelayer} from "../contracts/relayer/IDataverseRelayer.sol";
import {DataverseRelayer} from "../contracts/relayer/DataverseRelayer.sol";
import {DataverseVerifier} from "../contracts/verifier/DataverseVerifier.sol";
import {DataverseToken} from "../contracts/token/DataverseToken.sol";
import "forge-std/Test.sol";

contract ExtensionTest is Test {
    // Client
    event RequestCanceled(bytes32 requestId);
    event RequestFulfilled(bytes32 requestId);

    // Relayer
    event ResponseSent(bytes32 indexed requestId, address submitter);

    DataverseToken dataverseToken;
    DataverseVerifier dataverseVerifier;
    DataverseRelayer dataverseRelayer;
    DataverseClientMock dataverseClientMock;

    address owner;
    address requestSender;
    address responseSubmitter;

    bytes32 imageId = bytes32("imageId");
    string payload = "{}";

    uint256 offChainValue = 100;

    function setUp() public {
        owner = makeAddr("owner");
        requestSender = makeAddr("requestSender");
        responseSubmitter = makeAddr("responseSubmitter");

        vm.startPrank(owner);
        dataverseToken = new DataverseToken(owner, owner);
        dataverseRelayer = new DataverseRelayer(
            address(dataverseToken)
        );
        dataverseVerifier = new DataverseVerifier(address(dataverseRelayer));
        dataverseRelayer.setDataverseVerifier(address(dataverseVerifier));
        dataverseClientMock = new DataverseClientMock(
            address(dataverseToken),
            address(dataverseRelayer)
        );

        dataverseToken.mint(address(dataverseClientMock), dataverseClientMock.PAYMENT());
        dataverseVerifier.authorizeSubmitter(responseSubmitter, true);
        vm.stopPrank();
    }

    function test_WhenFulfillSuccessfully() public {
        vm.recordLogs();
        dataverseClientMock.requestOffChainValue(imageId, payload);

        bytes32 requestId;
        IDataverseRelayer.RequestParams memory requestParams;
        uint256 payment;
        uint256 expiration;

        Vm.Log[] memory entries = vm.getRecordedLogs();
        for (uint256 i = 0; i < entries.length; ++i) {
            if (entries[i].topics[0] == IDataverseRelayer.RequestReceived.selector) {
                requestId = entries[i].topics[1];
                (payment, expiration, requestParams) =
                    abi.decode(entries[i].data, (uint256, uint256, IDataverseRelayer.RequestParams));
            }
        }

        assertEq(uint8(dataverseClientMock.getRequestStatus(requestId)), uint8(IDataverseClient.RequestStatus.Pending));

        vm.expectEmit(true, true, true, true, address(dataverseClientMock));
        emit RequestFulfilled(requestId);
        vm.expectEmit(true, true, true, true, address(dataverseRelayer));
        emit ResponseSent(requestId, responseSubmitter);

        bytes memory responseData = abi.encode(offChainValue);
        vm.prank(responseSubmitter);
        bool success = dataverseVerifier.submit(requestId, requestParams, payment, expiration, responseData);
        assertTrue(success);
        assertEq(dataverseClientMock.offChainValue(), offChainValue);
        assertEq(dataverseToken.balanceOf(responseSubmitter), dataverseClientMock.PAYMENT());
    }

    function test_WhenRequestCanceled() public {
        vm.recordLogs();
        dataverseClientMock.requestOffChainValue(imageId, payload);

        assertEq(dataverseToken.balanceOf(address(dataverseClientMock)), 0);

        bytes32 requestId;
        uint256 payment;
        uint256 expiration;
        IDataverseRelayer.RequestParams memory requestParams;

        Vm.Log[] memory entries = vm.getRecordedLogs();
        for (uint256 i = 0; i < entries.length; ++i) {
            if (entries[i].topics[0] == IDataverseRelayer.RequestReceived.selector) {
                requestId = entries[i].topics[1];
                (payment, expiration, requestParams) =
                    abi.decode(entries[i].data, (uint256, uint256, IDataverseRelayer.RequestParams));
            }
        }

        assertEq(uint8(dataverseClientMock.getRequestStatus(requestId)), uint8(IDataverseClient.RequestStatus.Pending));

        vm.warp(block.timestamp + dataverseRelayer.EXPIRATION_INTERVAL());
        vm.expectEmit(true, true, true, true, address(dataverseClientMock));
        emit RequestCanceled(requestId);
        vm.prank(requestSender);
        dataverseClientMock.cancelRequestOffChainValue(requestId, payment, expiration, requestParams);

        assertEq(uint8(dataverseClientMock.getRequestStatus(requestId)), uint8(IDataverseClient.RequestStatus.Disabled));
        assertEq(dataverseToken.balanceOf(address(dataverseClientMock)), dataverseClientMock.PAYMENT());
    }
}
