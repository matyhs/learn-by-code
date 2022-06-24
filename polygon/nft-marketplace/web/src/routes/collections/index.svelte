<script lang="ts">
    import { ethers } from 'ethers';
    import Web3Modal from 'web3modal';
    import MarketPlace from '../../../../dist/artifacts/contracts/Marketplace.sol/Marketplace.json';
    import { marketplaceAddress } from '../../../../dist/config';
    import { onMount } from 'svelte';
    import { MarketItem } from '$lib/types/class';
    import type { SolMarketItem } from '$lib/types/type';
    import { asHttp, baseHTTP } from '$lib/scripts/ipfs';
    import Card from '$lib/components/card.svelte';
    import { accountCache } from '$lib/scripts/account';

    let image: HTMLImageElement;
    let dialog: HTMLDialogElement;
    let name: HTMLInputElement;
    let description: HTMLTextAreaElement;
    let priceInput: HTMLInputElement;
    let promise: Promise<MarketItem[]> | null;
    let tokenId: number;

    $: $accountCache ? loadNft() : promise = null;

    onMount(async () => {
        if ($accountCache) {
            await loadNft();
        }
    });

    const loadNft = async () => {
        const modal = new Web3Modal();
        const connection = await modal.connect();
        const provider = new ethers.providers.Web3Provider(connection); 
        const signer = provider.getSigner();
        const contract = new ethers.Contract(marketplaceAddress, MarketPlace.abi, signer);
        
        promise = contract.fetchMyItems().then((data: SolMarketItem[]) => {
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

    const openNewArtDialog = (event: CustomEvent) => {
        if (typeof dialog?.showModal === "function") {
            dialog?.showModal();
            dialog?.addEventListener('close', () => {
                if (!CSS.supports('backdrop-filter:blur(0px)')) {
                    const firstDiv = document.getElementById("baseParent");
                    firstDiv?.classList.remove("blur-md");
                }
                
                priceInput.value = "";
                name.value = "";
                description.value = "";
                image.src = "";
            });
        }

        if (!CSS.supports('backdrop-filter:blur(0px)')) {
            const firstDiv = document.getElementById("baseParent");
            firstDiv?.classList.add("blur-md");
        }

        priceInput.value = event.detail.item.price;
        name.value = event.detail.item.name;
        description.value = event.detail.item.description;
        image.src = event.detail.item.image;
        tokenId = event.detail.item.tokenId;
    };

    const sell = async () => {
        const priceString = priceInput.value;
        const modal = new Web3Modal();
        const connection = await modal.connect();
        const provider = new ethers.providers.Web3Provider(connection); 
        const signer = provider.getSigner();
        const contract = new ethers.Contract(marketplaceAddress, MarketPlace.abi, signer);
        const price = ethers.utils.parseEther(priceString);
        const listingPrice = await contract.getListingPrice();
        const transaction = await contract.resellToken(tokenId, price, { value: listingPrice });
        await transaction.wait();
        await loadNft();
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
            <Card {marketItem} account={$accountCache} transaction="Sell" on:transact={openNewArtDialog}/>
            {/each}
        {/await}
    </div>
    {/if}
</div>

<dialog bind:this={dialog} class="w-1/4 h-2/3 rounded-lg">
    <form method="dialog" class="font-mono flex flex-col w-full h-full">
        <section class="w-full h-full">
            <div class="border border-dashed border-brand-primary flex items-center justify-center h-1/3 min-h-[24rem] rounded-md cursor-pointer">
                <img bind:this={image} alt="Unable to load art" class="text-brand-primary/60" />
            </div>
            <div>
                <label for="artPrice" class="block">Price:</label>
                <input bind:this={priceInput} id="artPrice" type="number" class="block w-full border" step="0.1" placeholder="ETH" />
            </div>
            <div>
                <label for="artName" class="block">Name:</label>
                <input disabled bind:this={name} id="artName" type="text" class="block w-full border" />
            </div>
            <div class="h-1/3">
                <label for="artDesc" class="block">Description:</label>
                <textarea disabled bind:this={description} id="artDesc" class="block w-full h-full border resize-none"/>
            </div>
        </section>
        <menu>
            <button type="reset">Cancel</button>
            <button type="submit" on:click={sell}>Sell</button>
        </menu>
    </form>
</dialog>

<style>
    dialog::backdrop {
        backdrop-filter: blur(0.5rem);
    }
</style>