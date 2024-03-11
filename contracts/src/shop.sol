// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "@openzeppelin/contracts/access/Ownable.sol";

contract Shop is Ownable {
    struct SubStatus {
        uint256 createdAt;
        uint256 startedAt;
        uint256 expiredAt;
    }

    struct ResellerInfo {
        uint256 startedAt;
        uint256 ratio;
        uint256 commission;
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

    address public treasuryAddr;
    mapping(bytes32 => mapping(address => SubStatus)) internal subStatusMap;
    mapping(bytes32 => Item) internal itemMap;

    mapping(bytes32 => mapping(address => ResellerInfo))
        internal resellerInfoMap;

    event ProductCreated(bytes32 indexed pid, address indexed createdBy);
    event ProductPriceUpdated(
        bytes32 indexed pid,
        address indexed updatedBy,
        uint256 price
    );

    event BuyOK(address indexed from, bytes32 indexed pid);

    constructor(address treasuryAddr_) {
        treasuryAddr = treasuryAddr_;
    }

    function pay(
        bytes32 pid,
        uint256 receivables,
        address referrer
    ) private {
        // Is the payment amount sufficient
        Item memory itemInfo = itemMap[pid];
        require(receivables >= itemInfo.price, "Shop: insufficient amount");
        ResellerInfo storage reseller = resellerInfoMap[itemInfo.appId][
            referrer
        ];

        uint256 commission = 0;
        // Is the referrer valid
        if (reseller.startedAt > 0) {
            commission = (itemInfo.price * reseller.ratio) / 100;
            reseller.commission = reseller.commission + commission;

            // transfer
            transferMoney(referrer, commission);
        }

        transferMoney(treasuryAddr, itemInfo.price - commission);
    }

    function transferMoney(address recipient_, uint256 amount) private {
        address payable recipient = payable(recipient_);

        // solhint-disable-next-line avoid-low-level-calls, avoid-call-value
        (bool success, ) = recipient.call{value: amount}("");
        require(success, "Shop: transferMoney faild");
    }

    function subscribe(
        bytes32 pid,
        address beneficiary,
        address referrer
    ) public payable {
        // money ok
        pay(pid, msg.value, referrer);
        Item memory itemInfo = itemMap[pid];
        SubStatus storage status = subStatusMap[itemInfo.appId][beneficiary];

        // solhint-disable-next-line not-rely-on-time
        uint256 currentTime = block.timestamp;

        if (status.createdAt == 0) {
            status.createdAt = currentTime;
        }

        if (status.startedAt == 0 || status.expiredAt < currentTime) {
            status.startedAt = currentTime;
            status.expiredAt = currentTime + itemInfo.duration;
        } else {
            status.expiredAt = status.expiredAt + itemInfo.duration;
        }

        emit BuyOK(msg.sender, pid);
    }

    function buy(bytes32 pid, address referrer) public payable {
        pay(pid, msg.value, referrer);

        emit BuyOK(msg.sender, pid);
    }

    function querySub(bytes32 appId, address subscriber)
        public
        view
        returns (SubStatus memory status, bool isActive)
    {
        status = subStatusMap[appId][subscriber];

        // solhint-disable-next-line not-rely-on-time
        isActive = status.expiredAt > block.timestamp;
    }

    // Product
    function createProduct(
        bytes32 pid,
        bytes32 appId,
        uint256 price,
        uint256 payType,
        uint256 duration
    ) public onlyOwner {
        require(itemMap[pid].createdAt == 0, "Product: Product pid conflict");
        itemMap[pid].pid = pid;
        itemMap[pid].appId = appId;
        itemMap[pid].price = price;
        itemMap[pid].payType = payType;
        itemMap[pid].duration = duration;

        // solhint-disable-next-line not-rely-on-time
        itemMap[pid].createdAt = block.timestamp;
        itemMap[pid].createdBy = msg.sender;
        itemMap[pid].updatedBy = msg.sender;

        emit ProductCreated(pid, msg.sender);
    }

    function updatePrice(bytes32 pid, uint256 price) public onlyOwner {
        require(itemMap[pid].createdAt > 0, "Product: Product does not exist");
        itemMap[pid].price = price;
        emit ProductPriceUpdated(pid, msg.sender, price);
    }

    function queryProduct(bytes32 id) public view returns (Item memory item) {
        item = itemMap[id];
    }

    // Reseller
    function addReseller(
        address reseller,
        bytes32 appId,
        uint256 ratio
    ) public onlyOwner {
        require(
            resellerInfoMap[appId][reseller].startedAt == 0,
            "Shop: Already added"
        );

        // solhint-disable-next-line not-rely-on-time
        resellerInfoMap[appId][reseller].startedAt = block.timestamp;
        resellerInfoMap[appId][reseller].ratio = ratio;
    }

    function queryReseller(address reseller, bytes32 appId)
        public
        view
        returns (ResellerInfo memory resellerInfo)
    {
        resellerInfo = resellerInfoMap[appId][reseller];
    }

    fallback() external payable {}

    // solhint-disable-next-line no-empty-blocks
    receive() external payable {}
}
