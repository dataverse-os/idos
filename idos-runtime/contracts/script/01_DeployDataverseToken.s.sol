// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseToken} from "../contracts/token/DataverseToken.sol";
import "forge-std/Script.sol";

contract DeployDataverseToken is Script {
    address treasury = 0x3F3786B67DC1874C3Bd8e8CD61F5eea87604470F;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseToken dataverseToken = new DataverseToken(vm.addr(deployerPrivateKey), treasury);
        vm.stopBroadcast();

        console.log("DataverseToken:", address(dataverseToken));
    }
}
