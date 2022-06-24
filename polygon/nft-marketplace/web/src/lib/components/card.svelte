<script lang="ts">
    import type { MarketItem } from "$lib/types/class";
    import { createEventDispatcher } from 'svelte';

    export let marketItem: MarketItem;
    export let account: string | null;
    export let transaction: string;

    const dispatch = createEventDispatcher();

    let hoverState = false;
    let canBuy = false;

    $: canBuy = hoverState && account != null && account !== marketItem.seller;

    const enter = () => {
        hoverState = true;
    };

    const leave = () => {
        hoverState = false;
    }

    const transact = () => {
        dispatch('transact', {
            item: marketItem
        });
    }
</script>

<div class="rounded-lg drop-shadow-lg shadow-lg w-[18rem] max-w-1/5 h-[24rem] max-h-1/3 relative transition-transform duration-150 hover:scale-105" on:mouseenter={enter} on:mouseleave={leave}>
    <div class="h-2/3 rounded-t-lg bg-brand-secondary">
        <img class="object-cover" alt="Art by" src="{marketItem.image}"/>
    </div>
    <div class="h-1/3 bg-brand-primary rounded-b-lg flex { canBuy ? "" : "flex-col"} p-2 drop-shadow-lg">
        {#if canBuy}
            <div class="rounded-full bg-brand-tertiary/80 shadow-2xl px-4 py-2 self-center w-full cursor-pointer text-center" on:click={transact}>
                <span class="text-brand-secondary cursor-pointer">{transaction}</span>
            </div>
        {:else}
            <span class="font-semibold text-lg text-brand-secondary">{marketItem.name}</span>
            <span class="text-lg text-brand-tertiary">{marketItem.price} ETH</span>
            <span class="text-xs text-brand-secondary text-opacity-70 mt-2">{marketItem.description}</span>
        {/if}
    </div>
</div>