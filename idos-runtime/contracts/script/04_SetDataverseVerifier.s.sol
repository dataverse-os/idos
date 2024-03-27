// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseRelayer} from "../contracts/relayer/DataverseRelayer.sol";
import "forge-std/Script.sol";

contract SetDataverseVerifier is Script {
    address dataverseRelayer = 0xeD9F9d3Aa9C8F8ea6F1a9Caf34a5a70342172d69;
    address dataverseVerifier = 0xD1C27Ae64f08BCdC77F43A6613CCa0260998077A;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseRelayer(dataverseRelayer).setDataverseVerifier(dataverseVerifier);
        vm.stopBroadcast();
    }
}
