// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC677Receiver} from "./IERC677Receiver.sol";

abstract contract ERC677Sender is ERC20 {
    function transferAndCall(address to, uint256 value, bytes calldata data) public returns (bool success) {
        transfer(to, value);
        if (_isContract(to)) {
            _contractFallback(to, value, data);
        }
        return true;
    }

    function _isContract(address _addr) private view returns (bool hasCode) {
        uint256 length;
        assembly {
            length := extcodesize(_addr)
        }
        return length > 0;
    }

    function _contractFallback(address to, uint256 value, bytes calldata data) private {
        IERC677Receiver receiver = IERC677Receiver(to);
        receiver.onTokenTransfer(msg.sender, value, data);
    }
}
