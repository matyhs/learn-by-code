<script lang="ts">
    import { ethers } from 'ethers';
    import Web3Modal from 'web3modal';
    import MarketPlace from '../../../dist/artifacts/contracts/Marketplace.sol/Marketplace.json';
    import { marketplaceAddress } from '../../../dist/config';
    import { onMount } from 'svelte';
    import { MarketItem } from '$lib/types/class';
    import type { SolMarketItem } from '$lib/types/type';
    import { asHttp, baseHTTP } from '$lib/scripts/ipfs';
    import Card from '$lib/components/card.svelte';
    import { accountCache } from '$lib/scripts/account';

    let promise: Promise<MarketItem[]>;

    onMount(async () => {
        await loadNft();
    });

    const loadNft = async () => {
        const provider = new ethers.providers.JsonRpcProvider("https://matic-mumbai.chainstacklabs.com");
        const contract = new ethers.Contract(marketplaceAddress, MarketPlace.abi, provider);
        
        promise = contract.fetchMarketItems().then((data: SolMarketItem[]) => {
            return Promise.all(
                        data.map(async (item) => {
                            const tokenUri: string = await contract.tokenURI(item.tokenId);
                            const metadataUrl = asHttp(baseHTTP, tokenUri);
                            const response = await fetch(metadataUrl);
                            const metadata = await response.json();
                            const price = ethers.utils.formatUnits(item.price.toString(), 'ether');
                            return new MarketItem(item.tokenId, price, item.owner, item.seller, metadata.name, metadata.description, asHttp(baseHTTP, metadata.image)); 
                        })
                    );
        });
    }

    const buy = async (event: CustomEvent) => {
        const modal = new Web3Modal();
        const connection = await modal.connect();
        const provider = new ethers.providers.Web3Provider(connection); 
        const signer = provider.getSigner();
        const contract = new ethers.Contract(marketplaceAddress, MarketPlace.abi, signer);

        const price = ethers.utils.parseUnits(event.detail.item.price, 'ether');
        const transaction = await contract.createMarketSale(event.detail.item.tokenId, { value: price});
        await transaction.wait();
        loadNft();
    };
</script>

<div class="font-mono w-full">
    {#if promise != null}
    <div class="grid grid-cols-5 gap-20 mt-4 px-2">
        {#await promise}
            {#each [...Array(10).keys()] as _}
                <div class="rounded-lg drop-shadow-lg shadow-lg w-[18rem] max-w-1/5 h-[24rem] max-h-1/3 relative animate-pulse">
                    <div class="h-2/3 rounded-t-lg bg-brand-primary p-2">
                        <div class="bg-brand-secondary/30 rounded-lg h-full">

                        </div>
                    </div>
                    <div class="h-1/3 bg-brand-primary rounded-b-lg flex flex-col space-y-4 p-2 drop-shadow-lg">
                        <div class="h-2 bg-brand-secondary/30 w-full rounded-lg"></div>
                        <div class="flex space-x-2">
                            <div class="h-2 bg-brand-secondary/30 w-2/3 rounded-lg"></div>
                            <div class="h-2 bg-brand-secondary/30 flex-1 rounded-lg"></div>
                        </div>
                        <div class="flex flex-col flex-1 w-full">
                            <div class="flex-1 bg-brand-secondary/30 w-full rounded-t-lg"></div>
                            <div class="h-4 bg-brand-secondary/30 w-1/2 rounded-bl-lg"></div>
                        </div>
                    </div>
                </div>
            {/each}
        {:then marketItems} 
            {#each marketItems as marketItem}
                <Card {marketItem} account={$accountCache} transaction="Buy" on:transact={buy}/>
            {/each}
        {/await}
    </div>
    {/if}
</div>