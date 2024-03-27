// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC677Sender} from "./ERC677/ERC677Sender.sol";

contract DataverseToken is ERC677Sender, ERC20Permit, Ownable {
    // Total supply of dataverse token is 100M
    uint256 public immutable TOTAL_SUPPLY = 100_000_000 * 10 ** uint256(decimals());

    constructor(address owner, address to)
        ERC20("Dataverse Token", "DVT")
        ERC20Permit("Dataverse Token")
        Ownable(owner)
    {
        _mint(to, TOTAL_SUPPLY);
    }

    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }
}
