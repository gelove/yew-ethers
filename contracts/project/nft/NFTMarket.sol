// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract NFT is ERC721URIStorage {
    using Counters for Counters.Counter;

    Counters.Counter private _counter;
    // address private minter; // 加一个minter
    address private marketplaceAddress;

    // constructor(address _marketplaceAddress, address _minter)
    //     ERC721("Metaverse", "NFT")
    // {
    //     minter = _minter;
    //     marketplaceAddress = _marketplaceAddress;
    // }

    constructor(address _marketplaceAddress) ERC721("Metaverse", "NFT") {
        marketplaceAddress = _marketplaceAddress;
    }

    function createToken(string memory tokenURI) external returns (uint256) {
        // require(minter == msg.sender, "Only minter can createToken");
        _counter.increment();
        uint256 newItemId = _counter.current();

        _mint(msg.sender, newItemId);
        _setTokenURI(newItemId, tokenURI);
        setApprovalForAll(marketplaceAddress, true);
        return newItemId;
    }
}

contract NFTMarket is ReentrancyGuard {
    using Counters for Counters.Counter;

    Counters.Counter private _itemIds;
    Counters.Counter private _itemsSold;

    address payable private owner;
    uint256 private listingPrice = 0.025 ether; // 上市手续费

    constructor() {
        owner = payable(msg.sender);
    }

    struct MarketItem {
        uint256 itemId;
        address nftContract;
        uint256 tokenId; // nft资产坐标信息, 地址或其他能标识位置的信息
        address payable seller;
        address payable owner;
        uint256 price;
        bool sold;
    }

    mapping(uint256 => MarketItem) private idToMarketItem;

    MarketItem[] history;

    event MarketItemCreated(
        uint256 indexed itemId,
        address indexed nftContract,
        uint256 indexed tokenId,
        address seller,
        address owner,
        uint256 price,
        bool sold
    );

    /* Returns the listing price of the contract */
    function getListingPrice() public view returns (uint256) {
        return listingPrice;
    }

    // 售卖 需要上架手续费 listingPrice
    /* Places an item for sale on the marketplace */
    function createMarketItem(address nftContract, uint256 tokenId, uint256 price) external payable nonReentrant {
        require(price > 0, "Price must be at least 1 wei");
        require(msg.value == listingPrice, "Price must be equal to listing price");

        _itemIds.increment();
        uint256 itemId = _itemIds.current();
        // 索引从1开始
        idToMarketItem[itemId] =
            MarketItem(itemId, nftContract, tokenId, payable(msg.sender), payable(address(0)), price, false);

        IERC721(nftContract).transferFrom(msg.sender, address(this), tokenId);

        emit MarketItemCreated(itemId, nftContract, tokenId, msg.sender, address(0), price, false);
    }

    /* 购买 */
    /* Creates the sale of a marketplace item */
    /* Transfers ownership of the item, as well as funds between parties */
    function createMarketSale(address nftContract, uint256 itemId) external payable nonReentrant {
        MarketItem memory item = idToMarketItem[itemId];
        // uint256 price = idToMarketItem[itemId].price;
        // uint256 tokenId = idToMarketItem[itemId].tokenId;
        require(msg.value == item.price, "Please submit the asking price in order to complete the purchase");

        item.sold = true; // 标识已售出
        item.owner = payable(msg.sender); // 资产所有者转移
        item.seller.transfer(msg.value); // 给售卖者转钱
        idToMarketItem[itemId] = item;
        IERC721(nftContract).transferFrom(address(this), msg.sender, item.tokenId); // 从市场转移至购买者
        _itemsSold.increment(); // 售出数量增加
        payable(owner).transfer(listingPrice); // 给市场所有者转钱
    }

    /* Returns all unsold market items */
    function fetchMarketItems() public view returns (MarketItem[] memory) {
        uint256 itemCount = _itemIds.current();
        uint256 unsoldItemCount = _itemIds.current() - _itemsSold.current();
        uint256 currentIndex = 0;

        MarketItem[] memory items = new MarketItem[](unsoldItemCount);
        for (uint256 i = 0; i < itemCount; i++) {
            if (idToMarketItem[i + 1].owner == address(0)) {
                uint256 currentId = i + 1;
                MarketItem storage currentItem = idToMarketItem[currentId];
                items[currentIndex] = currentItem;
                currentIndex += 1;
            }
        }
        return items;
    }

    function fetchMarketItemsByStart(uint256 start, uint256 size)
        public
        view
        returns (uint256, uint256, MarketItem[] memory)
    {
        uint256 total = history.length;
        if (total == 0) {
            return (total, 0, new MarketItem[](0));
        }

        // 每页最多10条
        if (size > 10 || size == 0) {
            size = 10;
        }
        if (start > total - 1 || start == 0) {
            start = total - 1;
        }
        if (size > start + 1) {
            size = start + 1;
        }

        MarketItem[] memory list = new MarketItem[](size);
        uint256 end = start + 1 - size;
        for (uint256 i = start; i >= end; i--) {
            list[i] = history[i];
        }
        return (total, end, list);
    }

    function fetchMarketItemsByPage(uint256 page, uint256 size)
        public
        view
        returns (uint256, uint256, MarketItem[] memory)
    {
        uint256 total = history.length;
        if (total == 0 || total <= page * size) {
            return (total, 0, new MarketItem[](0));
        }

        uint256 start = total - page * size - 1;
        return fetchMarketItemsByStart(start, size);
    }

    /* Returns only items that a user has purchased */
    function fetchMyNFTs() public view returns (MarketItem[] memory) {
        uint256 totalItemCount = _itemIds.current();
        uint256 itemCount = 0;
        uint256 currentIndex = 0;

        for (uint256 i = 0; i < totalItemCount; i++) {
            if (idToMarketItem[i + 1].owner == msg.sender) {
                itemCount += 1;
            }
        }

        MarketItem[] memory items = new MarketItem[](itemCount);
        for (uint256 i = 0; i < totalItemCount; i++) {
            if (idToMarketItem[i + 1].owner == msg.sender) {
                uint256 currentId = i + 1;
                MarketItem storage currentItem = idToMarketItem[currentId];
                items[currentIndex] = currentItem;
                currentIndex += 1;
            }
        }
        return items;
    }

    /* Returns only items a user has created */
    function fetchItemsCreated() public view returns (MarketItem[] memory) {
        uint256 totalItemCount = _itemIds.current();
        uint256 itemCount = 0;
        uint256 currentIndex = 0;

        for (uint256 i = 0; i < totalItemCount; i++) {
            if (idToMarketItem[i + 1].seller == msg.sender) {
                itemCount += 1;
            }
        }

        MarketItem[] memory items = new MarketItem[](itemCount);
        for (uint256 i = 0; i < totalItemCount; i++) {
            if (idToMarketItem[i + 1].seller == msg.sender) {
                uint256 currentId = i + 1;
                MarketItem storage currentItem = idToMarketItem[currentId];
                items[currentIndex] = currentItem;
                currentIndex += 1;
            }
        }
        return items;
    }
}
