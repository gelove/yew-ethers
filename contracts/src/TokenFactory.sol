// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";

contract CustomToken is ERC20, Ownable {
    using Address for address payable;

    uint8 private _decimals;

    event Receive(address indexed sender, uint256 value);
    event Fallback(address indexed sender, uint256 value, bytes data);
    event Withdraw(address indexed sender, address indexed token, uint256 amount);

    constructor(string memory name_, string memory symbol_, uint8 decimals_, uint256 initialSupply_, address owner_)
        ERC20(name_, symbol_)
    {
        _decimals = decimals_;
        _mint(owner_, initialSupply_ * 10 ** uint256(decimals_));
        transferOwnership(owner_);
    }

    function destroy() external onlyOwner {
        selfdestruct(payable(msg.sender));
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    receive() external payable {
        emit Receive(msg.sender, msg.value);
    }

    fallback() external payable {
        emit Fallback(msg.sender, msg.value, msg.data);
    }

    function withdraw() external onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "insufficient balance");
        payable(msg.sender).sendValue(balance);
        emit Withdraw(msg.sender, address(0), balance);
    }
}

contract TokenFactory is Ownable {
    using Address for address payable;

    event Receive(address indexed sender, uint256 value);
    event Fallback(address indexed sender, uint256 value, bytes data);
    event Withdraw(address indexed owner, address indexed token, uint256 amount);
    event TokenCreated(
        address indexed owner,
        address indexed token,
        string indexed name,
        string symbol,
        uint8 decimals,
        uint256 initialSupply
    );

    receive() external payable {
        emit Receive(msg.sender, msg.value);
    }

    fallback() external payable {
        emit Fallback(msg.sender, msg.value, msg.data);
    }

    function destroy() external onlyOwner {
        selfdestruct(payable(msg.sender));
    }

    function createToken(string calldata name, string calldata symbol, uint8 decimals, uint256 initialSupply)
        external
        payable
    {
        // 手续费必须大于等于 0.005 个主币
        // require(msg.value >= 5 * 10 ** uint256(15), "Fee must >= 0.005 native token");
        // require(msg.value >= 0.005 ether, "fee must >= 0.005 ETH");
        CustomToken token = new CustomToken(name, symbol, decimals, initialSupply, msg.sender);
        emit TokenCreated(token.owner(), address(token), name, symbol, decimals, initialSupply);
    }

    function withdraw() public onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "insufficient balance");
        payable(msg.sender).sendValue(balance);
        emit Withdraw(msg.sender, address(0), balance);
    }
}
