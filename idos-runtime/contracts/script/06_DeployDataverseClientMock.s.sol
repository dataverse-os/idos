// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {DataverseClientMock} from "../test/mocks/DataverseClientMock.sol";
import "forge-std/Script.sol";

contract DeployDataverseClientMock is Script {
    address dataverseToken = 0xC5392Dc6b65dFc4842fEe44691AF863538BbB966;
    address dataverseRelayer = 0xeD9F9d3Aa9C8F8ea6F1a9Caf34a5a70342172d69;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        DataverseClientMock dataverseClientMock = new DataverseClientMock(dataverseToken, dataverseRelayer);
        vm.stopBroadcast();

        console.log("DataverseClientMock:", address(dataverseClientMock));
    }
}
