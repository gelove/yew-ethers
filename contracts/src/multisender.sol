// SPDX-License-Identifier: MIT
// author https://biubiu.tools
pragma solidity ^0.8.0;

interface IERC20 {
    function balanceOf(address account) external view returns (uint256);

    function transfer(address to, uint256 amount) external returns (bool);

    function allowance(address owner, address spender)
        external
        view
        returns (uint256);

    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);
}

struct SubStatus {
    uint256 createdAt;
    uint256 startedAt;
    uint256 expiredAt;
}

struct Item {
    bytes32 pid;
    bytes32 appId;
    uint256 price;
    uint256 payType; // 0 => Product   1 => subscription plan
    uint256 duration; // if Product: duration == 0  else plan: duration > 0
    uint256 createdAt;
    address createdBy;
    address updatedBy;
}

interface IShop {
    function querySub(bytes32 appId, address subscriber)
        external
        view
        returns (SubStatus memory status, bool isActive);

    function buy(bytes32 pid, address referrer) external payable;

    function queryProduct(bytes32 id) external view returns (Item memory item);
}

contract Multisender {
    address public shopAddr;

    event MultisendOK(address indexed from);

    constructor(address shopAddr_) {
        shopAddr = shopAddr_;
    }

    function checkLen(uint256 recipientsLen, uint256 amountsLen) private pure {
        require(recipientsLen == amountsLen, "Multisender: length not match");
    }

    function checkEtherBalance(uint256 moneyin, uint256 moneyneed)
        private
        pure
    {
        require(moneyin >= moneyneed, "Multisender: insufficient ether");
    }

    function checkSubscriptions(address target, address referrer) private {
        // appid multisender
        // bytes32
        bytes32 appId = 0x6d756c746973656e646572000000000000000000000000000000000000000000;
        IShop shop = IShop(shopAddr);
        (, bool isActive) = shop.querySub(appId, target);

        // pid multisender-ppv
        bytes32 pid = 0x6d756c746973656e6465722d7070760000000000000000000000000000000000;

        // buy
        if (!isActive) {
            Item memory item = shop.queryProduct(pid);
            shop.buy{value: item.price * 1 wei}(pid, referrer);
        }
    }

    function checkTokenBalance(
        address token,
        address target,
        uint256 tokenneed
    ) private view {
        IERC20 eRC20Token = IERC20(token);

        // solhint-disable reason-string
        require(
            eRC20Token.balanceOf(target) >= tokenneed,
            "Multisender: insufficient token balance"
        );

        // solhint-disable reason-string
        require(
            eRC20Token.allowance(target, address(this)) >= tokenneed,
            "Multisender: insufficient token allowance"
        );
    }

    function multisendToken(
        address token,
        address[] memory recipients,
        uint256[] memory amounts,
        uint256 total,
        address referrer
    ) public payable {
        checkLen(recipients.length, amounts.length);
        checkTokenBalance(token, msg.sender, total);
        checkSubscriptions(msg.sender, referrer);
        IERC20 eRC20Token = IERC20(token);
        bulksendToken(eRC20Token, recipients, amounts);
        emit MultisendOK(msg.sender);
    }

    function multisendEther(
        address[] memory recipients,
        uint256[] memory amounts,
        uint256 total,
        address referrer
    ) public payable {
        checkLen(recipients.length, amounts.length);
        checkEtherBalance(msg.value, total);
        checkSubscriptions(msg.sender, referrer);
        bulksendEther(recipients, amounts);
        emit MultisendOK(msg.sender);
    }

    function bulksendEther(address[] memory receivers, uint256[] memory amounts)
        internal
    {
        for (uint256 i = 0; i < receivers.length; i++) {
            address payable recipient = payable(address(receivers[i]));

            // solhint-disable-next-line avoid-low-level-calls, avoid-call-value
            (bool success, ) = recipient.call{value: amounts[i]}("");
            require(success, "Multisender: unable to send value");
        }
    }

    function bulksendToken(
        IERC20 eRC20Token,
        address[] memory receivers,
        uint256[] memory amounts
    ) internal {
        for (uint256 i = 0; i < receivers.length; i++) {
            eRC20Token.transferFrom(msg.sender, receivers[i], amounts[i]);
        }
    }

    fallback() external payable {}

    // solhint-disable-next-line no-empty-blocks
    receive() external payable {}
}
