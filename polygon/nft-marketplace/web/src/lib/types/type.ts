type SolMarketItem = {
    tokenId: number;
    seller: string;
    owner: string;
    price: number;
    sold: boolean;
}

type IPFSMetadata = {
    url: string;
}

export type { SolMarketItem, IPFSMetadata }