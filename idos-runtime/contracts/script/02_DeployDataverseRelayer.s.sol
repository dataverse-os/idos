// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseRelayer} from "../contracts/relayer/DataverseRelayer.sol";
import "forge-std/Script.sol";

contract DeployDataverseRelayer is Script {
    address dataverseToken = 0xC5392Dc6b65dFc4842fEe44691AF863538BbB966;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseRelayer dataverseRelayer = new DataverseRelayer(dataverseToken);
        vm.stopBroadcast();

        console.log("DataverseRelayer:", address(dataverseRelayer));
    }
}
