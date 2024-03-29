// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseClientMock} from "../test/mocks/DataverseClientMock.sol";
import "forge-std/Script.sol";

contract ClientMockRequest is Script {
    address dataverseClientMock = 0xB5c9749785D622872f517e740a96BCAB284e9266;

    function run() public view {
        uint256 offChainValue = DataverseClientMock(dataverseClientMock).offChainValue();
        console.log("Value:", offChainValue);
    }
}
