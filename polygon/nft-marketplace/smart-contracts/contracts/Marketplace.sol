//SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

import "hardhat/console.sol";

contract Marketplace is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    Counters.Counter private _itemsSold;

    uint256 constant private LISTING_PRICE = 0.025 ether;
    address payable immutable private owner;
    mapping(uint256 => MarketItem) private marketItemHashTable;

    struct MarketItem {
        uint256 tokenId;
        address payable seller;
        address payable owner;
        uint256 price;
        bool sold;
    }

    event MarketItemCreated (
        uint256 indexed tokenId,
        address seller,
        address owner,
        uint256 price,
        bool sold
    );

    constructor() ERC721("Snowma Tokens", "SNM") {
        owner = payable(msg.sender);
    }

    function getOwnerAddress() public view returns(address) {
        return owner;
    }

    function getOwnerBalance() public view returns(uint256) {
        return owner.balance;
    }

    function getContractAddress() public view returns(address) {
        return address(this);
    }

    function getContractBalance() public view returns(uint256) {
        return address(this).balance;
    }

    function getListingPrice() public pure returns(uint256) {
        return LISTING_PRICE;
    }

    function createToken(
        string calldata tokenURI, 
        uint256 price
    ) public payable returns(uint256) {
        _tokenIds.increment();
        uint256 newTokenId = _tokenIds.current();

        _mint(msg.sender, newTokenId);
        _setTokenURI(newTokenId, tokenURI);
        createMarketItem(newTokenId, price);

        return _tokenIds.current();
    }

    function createMarketItem(
        uint256 tokenId,
        uint256 price
    ) private {
        require(price > 0, "Price must be at least 1 wei");
        require(msg.value == LISTING_PRICE, "Invalid listing payment");

        marketItemHashTable[tokenId] = MarketItem(
            tokenId,
            payable(msg.sender),
            payable(address(this)),
            price,
            false
        );

        _transfer(msg.sender, address(this), tokenId);

        emit MarketItemCreated(
            tokenId,
            msg.sender,
            address(this),
            price,
            false
        );
    }

    function resellToken(
        uint256 tokenId,
        uint256 price
    ) public payable {
        require(marketItemHashTable[tokenId].owner == msg.sender, "Only owned items can be sold");
        require(price > 0, "Price must be at least 1 wei");
        require(msg.value == LISTING_PRICE, "Invalid listing payment");

        marketItemHashTable[tokenId].price = price;
        marketItemHashTable[tokenId].owner = payable(address(this));
        marketItemHashTable[tokenId].seller = payable(msg.sender);
        marketItemHashTable[tokenId].sold = false;
        _itemsSold.decrement();

        _transfer(msg.sender, address(this), tokenId);
    }

    function createMarketSale(
        uint256 tokenId
    ) public payable {
        require(!marketItemHashTable[tokenId].sold, "Item is not for sale");
        require(marketItemHashTable[tokenId].price == msg.value, "Invalid payment");
        
        address seller = marketItemHashTable[tokenId].seller;
        marketItemHashTable[tokenId].owner = payable(msg.sender);
        marketItemHashTable[tokenId].seller = payable(address(this));
        marketItemHashTable[tokenId].sold = true;
        _itemsSold.increment();

        _transfer(address(this), msg.sender, tokenId);
        owner.transfer(LISTING_PRICE);
        payable(seller).transfer(msg.value);
    }

    function fetchMarketItems() public view returns (MarketItem[] memory) {
        uint itemCount = _tokenIds.current();
        uint unsoldItemCount = itemCount - _itemsSold.current();
        uint index = 0;

        MarketItem[] memory items = new MarketItem[](unsoldItemCount);
        for (uint i = 1; i <= itemCount; i++) {
            if (!marketItemHashTable[i].sold) {
                items[index] = marketItemHashTable[i];
                index += 1;
            }
        }

        return items;
    }

    function fetchMyItems() public view returns (MarketItem[] memory) {
        uint itemCount = _tokenIds.current();
        uint userItemCount = 0;

        for (uint i = 1; i <= itemCount; i++) {
            if (marketItemHashTable[i].owner == msg.sender) {
                userItemCount += 1;
            }
        }

        MarketItem[] memory items = new MarketItem[](userItemCount);
        uint index = 0;

        for (uint i = 1; i <= itemCount; i++) {
            if (marketItemHashTable[i].owner == msg.sender) {
                items[index] = marketItemHashTable[i];
                index += 1;
            }
        }

        return items;
    }

    function fetchMyListedItems() public view returns(MarketItem[] memory) {
        uint itemCount = _tokenIds.current();
        uint userListedItemCount = 0;

        for (uint i = 1; i <= itemCount; i++) {
            if (marketItemHashTable[i].seller == msg.sender) {
                userListedItemCount += 1;
            }
        }

        MarketItem[] memory items = new MarketItem[](userListedItemCount);
        uint index = 0;

        for (uint i = 1; i <= itemCount; i++) {
            if (marketItemHashTable[i].seller == msg.sender) {
                items[index] = marketItemHashTable[i];
                index += 1;
            }
        }

        return items;
    }
}