// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseClientMock} from "../../test/extension/mocks/DataverseClientMock.sol";
import "forge-std/Script.sol";

contract ClientMockRequest is Script {
    address dataverseClientMock = 0xB5c9749785D622872f517e740a96BCAB284e9266;
    bytes32 imageId = 0x396e2323d8e5264d44ad141b1c34d0e74f2cc7b9cf26897a312a8e84850900f1;
    string payload = '{"modelId":"test-model-id"}';

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseClientMock(dataverseClientMock).requestOffChainValue(imageId, payload);
        vm.stopBroadcast();
    }
}
