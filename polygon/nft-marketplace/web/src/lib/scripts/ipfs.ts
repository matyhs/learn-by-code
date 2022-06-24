const baseIPFS = "ipfs://";
const baseHTTP = "https://ipfs.io/ipfs/"

const asHttp = (baseUrl: string, ipfs: string) => {
    const ipfsUrl = new URL(ipfs);
    return `${baseUrl}${ipfsUrl.hostname}${ipfsUrl.pathname}`;
};

const asIPFS = (baseUrl: string, http: string) => {
    const httpUrl = new URL(http);
    const pathName = httpUrl.pathname.substring(6);
    return `${baseUrl}${pathName}`
}

export { asHttp, asIPFS, baseIPFS, baseHTTP }