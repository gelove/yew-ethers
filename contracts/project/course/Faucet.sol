// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/utils/Address.sol";

contract Faucet {
    using Address for address payable;

    event Receive(address indexed sender, uint256 value);
    event Fallback(address indexed sender, uint256 value, bytes data);
    event Withdraw(address indexed recipient, uint256 amount);

    uint256 private _limit;

    constructor(uint256 limit) {
        _limit = limit;
    }

    receive() external payable {
        emit Receive(msg.sender, msg.value);
    }

    fallback() external payable {
        emit Fallback(msg.sender, msg.value, msg.data);
    }

    function setLimit(uint256 limit) public {
        _limit = limit;
    }

    function withdraw() external {
        uint256 balance = address(this).balance;
        require(balance >= 1 ether, "insufficient balance");
        payable(msg.sender).sendValue(1 ether);
        emit Withdraw(msg.sender, 1 ether);
    }
}
