// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/src/utils/SafeTransferLib.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
// import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Address.sol";
// import "@openzeppelin/contracts/utils/Counters.sol";
// import "@openzeppelin/contracts/utils/Multicall.sol";
// import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

contract Utils is Ownable, ReentrancyGuard {
    using Address for address;
    using Address for address payable;
    using SafeERC20 for IERC20;

    event Receive(address indexed sender, uint256 value);
    event Fallback(address indexed sender, uint256 value, bytes data);
    event Withdraw(address indexed recipient, address indexed token, uint256 amount);
    event Airdrop(address indexed sender, address indexed token, address[] accounts, uint256[] values);

    error NonContract();
    error ExcessLength(string);
    error InconsistentLength(string);
    error InsufficientBalance(string);
    error InsufficientAllowance(string);

    receive() external payable {
        emit Receive(msg.sender, msg.value);
    }

    fallback() external payable {
        emit Fallback(msg.sender, msg.value, msg.data);
    }

    /**
     * remix airdrop 测试
     * approve 0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f,2000000000000000000000000
     * allowance 0x38B48a6A8E7b6c1f9064553c4EA2Cc532D3D29Fc,0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f
     * airdrop 0x823a2BF3611226FC8f7c8840734A32c79643D21F,[0x20acc5427a362E6eB18D7824664075Ea166136c5],[2000000000000000000000]
     */
    function airdrop(address token, address[] calldata accounts, uint256[] calldata values)
        external
        payable
        nonReentrant
    {
        // require(msg.value >= 5 * 10 ** uint256(15), "Fee must greater than or equal to 0.005 native token");
        // require(accounts.length <= 255, "accounts.length must <= 255");
        // require(accounts.length == values.length, "accounts.length != values.length");
        if (accounts.length > 255) revert ExcessLength("accounts.length > 255");
        if (accounts.length != values.length) revert InconsistentLength("airdrop: accounts.length != values.length");

        uint256 total = 0;
        for (uint8 i = 0; i < accounts.length; i++) {
            assert(accounts[i] != address(0));
            total += values[i];
        }

        if (address(0) == token) {
            if (msg.value < total) revert InsufficientBalance("airdrop: insufficient balance");
            for (uint8 i = 0; i < accounts.length; i++) {
                // payable(accounts[i]).sendValue(values[i]);
                // 比 sendValue 少一步验证 应该节省gas 待测试
                (bool success,) = payable(accounts[i]).call{value: values[i]}("");
                require(success, "Address: unable to send value, recipient may have reverted");
            }
        } else {
            IERC20 _token = IERC20(token);
            if (_token.allowance(msg.sender, address(this)) < total) {
                revert InsufficientAllowance("airdrop: insufficient allowance");
            }
            if (_token.balanceOf(msg.sender) < total) revert InsufficientBalance("airdrop: insufficient balance");

            for (uint8 i = 0; i < accounts.length; i++) {
                _token.safeTransferFrom(msg.sender, accounts[i], values[i]);
            }
        }
        emit Airdrop(msg.sender, token, accounts, values);
    }

    /**
     * batch remove approve
     */
    function removeApprove(address user) external {}

    /**
     * Check the token balance of a wallet in a token contract
     * Pass 0x0 as a "token" address to get ETH balance.
     *
     * Returns the balance of the token for user. Avoids possible errors:
     * - return 0 on non-contract address
     * - returns 0 if the contract doesn't implement balanceOf
     */
    function tokenBalance(address user, address token) public view returns (uint256) {
        // // check if token is actually a contract
        // uint256 tokenCode;
        // // contract code size
        // assembly {
        //     tokenCode := extcodesize(token)
        // }
        // // is it a contract and does it implement balanceOf
        // if (tokenCode == 0) {
        //     return 0;
        // }
        // (bool canCall,) = token.call(abi.encodeWithSignature("balanceOf()"));
        // if (!canCall) {
        //     return 0;
        // }
        if (token == address(0)) {
            // native token
            return user.balance;
        }
        if (!token.isContract()) {
            return 0;
        }
        return IERC20(token).balanceOf(user);
    }

    /**
     * Check the balances for multiple tokens of a wallet.
     * Pass 0x0 as a "token" address to get ETH balance.
     *
     * Possible error throws:
     * - extremely large arrays for user and or tokens (gas cost too high)
     *
     * Returns a one-dimensional that's user.length * tokens.length long. The
     * array is ordered by all of the 0th users token balances, then the 1th
     * user, and so on.
     */
    function tokenBalances(address[] calldata users, address[] calldata tokens)
        external
        view
        returns (uint256[] memory)
    {
        uint256[] memory addrBalances = new uint256[](tokens.length * users.length);

        for (uint256 i = 0; i < users.length; i++) {
            for (uint256 j = 0; j < tokens.length; j++) {
                uint256 addrIdx = j + tokens.length * i;
                addrBalances[addrIdx] = tokenBalance(users[i], tokens[j]);
            }
        }

        return addrBalances;
    }

    function withdraw() public onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "insufficient balance");
        payable(msg.sender).sendValue(balance);
        emit Withdraw(msg.sender, address(0), balance);
    }

    function withdrawERC20(address token) public onlyOwner {
        IERC20 erc20 = IERC20(token);
        uint256 balance = erc20.balanceOf(address(this));
        require(balance > 0, "insufficient balance");
        erc20.safeTransfer(msg.sender, balance);
        emit Withdraw(msg.sender, token, balance);
    }
}
