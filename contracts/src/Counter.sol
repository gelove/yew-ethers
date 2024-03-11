// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;
    uint256 public number1;
    uint256 private number2;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number = number + 1;
    }

    // function double() public {
    //     // 判断溢出 应使用 SafeMath
    //     require(number < number * 2);
    //     number *= 2;
    // }
}
