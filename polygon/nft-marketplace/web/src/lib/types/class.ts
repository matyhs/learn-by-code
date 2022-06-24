class MarketItem {
    tokenId: number;
    price: string;
    owner: string;
    seller: string;
    name: string;
    description: string;
    image: string;

    constructor(tokenId: number, 
                price: string, 
                owner: string, 
                seller: string, 
                name: string, 
                description: string, 
                image: string) {
        this.tokenId = tokenId;
        this.price = price;
        this.owner = owner;
        this.seller = seller;
        this.name = name;
        this.description = description;
        this.image = image;
    }
}

class User {
    id: string;

    constructor(id: string) {
        this.id = id;
    }
}

export { User, MarketItem }