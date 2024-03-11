// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract A {
    event CallSuccess(string indexed a, bytes b);

    address public temp1;
    uint256 public temp2;

    function call_test(address addr) public {
        // abi.encodeWithSignature(string memory signature, ...) == abi.encodeWithSelector(bytes4(keccak256(bytes(signature))), ...)`
        (bool success, bytes memory result) = addr.call(abi.encodeWithSignature("test()"));
        // (bool success, bytes memory result) = addr.delegatecall(abi.encodeWithSignature("test()"));
        // (bool success, bytes memory result) = addr.staticcall(abi.encodeWithSignature("test()"));
        if (!success) {
            revert();
        }
        emit CallSuccess("test", result);
    }
}

contract B {
    address public temp1;
    uint256 public temp2;

    function test() public returns (string memory) {
        temp1 = msg.sender;
        temp2 = 10000;
        return "hello";
    }
}
