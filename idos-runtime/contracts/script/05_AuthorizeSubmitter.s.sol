// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseVerifier} from "../contracts/verifier/DataverseVerifier.sol";
import "forge-std/Script.sol";

contract AuthorizeSubmitter is Script {
    address dataverseVerifier = 0xD1C27Ae64f08BCdC77F43A6613CCa0260998077A;
    address submitter = 0xc490803bc98DAec6775132F54503331D8C79967e;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseVerifier(dataverseVerifier).authorizeSubmitter(submitter, true);
        vm.stopBroadcast();
    }
}
