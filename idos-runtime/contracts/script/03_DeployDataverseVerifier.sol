// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseVerifier} from "../contracts/verifier/DataverseVerifier.sol";
import "forge-std/Script.sol";

contract DeployDataverseVerifier is Script {
    address dataverseRelayer = 0xeD9F9d3Aa9C8F8ea6F1a9Caf34a5a70342172d69;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseVerifier dataverseVerifier = new DataverseVerifier(dataverseRelayer);
        vm.stopBroadcast();

        console.log("DataverseVerifier:", address(dataverseVerifier));
    }
}
