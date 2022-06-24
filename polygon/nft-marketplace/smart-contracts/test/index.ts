import { expect } from "chai";
import { ethers } from "hardhat";

describe("Greeter", function () {
  it("Should return the new greeting once it's changed", async function () {
    const Greeter = await ethers.getContractFactory("Greeter");
    const greeter = await Greeter.deploy("Hello, world!");
    await greeter.deployed();

    expect(await greeter.greet()).to.equal("Hello, world!");

    const setGreetingTx = await greeter.setGreeting("Hola, mundo!");

    // wait until the transaction is mined
    await setGreetingTx.wait();

    expect(await greeter.greet()).to.equal("Hola, mundo!");
  });
});

describe("Marketplace", function() {
  it("Should have contract balance", async function () {
    const Marketplace = await ethers.getContractFactory("Marketplace");
    const marketplace = await Marketplace.deploy();
    await marketplace.deployed();

    // test listing price
    const listingPrice = await marketplace.getListingPrice();
    expect(listingPrice).to.equal(ethers.utils.parseUnits('0.025', 'ether'));

    // test minting NFT
    const auctionPrice = ethers.utils.parseUnits('1', 'ether');
    const tokenId1 = await marketplace.createToken("https://www.somenfdomain.com/1", auctionPrice, { value: listingPrice });
    const tokenId2 = await marketplace.createToken("https://www.somenfdomain.com/2", auctionPrice, { value: listingPrice });
    // expect(tokenId1).to.equal(1);
    // expect(tokenId2).to.equal(2);

    // test list avaialble NFT
    const items = await marketplace.fetchMarketItems();
    expect(items.length).to.equal(2);

    // test buy NFT
    const [_, buyer] = await ethers.getSigners();
    await marketplace.connect(buyer).createMarketSale(1, { value: auctionPrice});

    // test my NFTs
    const myItems = await marketplace.connect(buyer).fetchMyItems();
    expect(myItems.length).to.equal(1);

    // test resell NFT
    const newAuctionPrice = ethers.utils.parseUnits('1.5', 'ether');
    marketplace.connect(buyer).resellToken(1, newAuctionPrice, { value: listingPrice });

    // test my NFT listings
    const myListings = await marketplace.connect(buyer).fetchMyListedItems();
    expect(myListings.length).to.equal(1);
  });
});