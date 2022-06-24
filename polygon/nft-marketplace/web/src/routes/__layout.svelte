<script context="module" lang="ts">
    export const load = (input: { session: App.Session }) => {
        const user = input.session.user;
        let account: string | null = null;

        if (user != null) {
            account = user.id;
        }

        return {
            props: {
                account
            }
        }
    }
</script>

<script lang="ts">
    import "../app.css";
    import Web3Modal from 'web3modal';
    import { ethers } from 'ethers';
    import { onMount } from 'svelte';
    import { accountCache } from '$lib/scripts/account';
    import { SIGN_MESSAGE } from '$lib/types/constants';

    export let account: string;
    let modal: Web3Modal;
    let accountChangedRegistered = false;

    onMount(async () => {
        modal = new Web3Modal({
            cacheProvider: true
        });

        modal.on("connect", (provider) => {
            if (!accountChangedRegistered) {
                provider.on("accountsChanged", (_: string[]) => {
                    disconnect();
                    connect();
                });
                accountChangedRegistered = true;
            }
        });

        if (account) {
            await modal.connect();
        }
        
        accountCache.set(account);
    });

    const connect = async () => {
        modal = new Web3Modal({
            cacheProvider: true
        });
        const connection = await modal.connect();
        const provider = new ethers.providers.Web3Provider(connection);
        const accounts = await provider.listAccounts();
        const signer = await provider.getSigner();
        const signedMessage = await signer.signMessage(SIGN_MESSAGE);
        
        if (accounts) {
            const response = await fetch('/api/account/signin', {
                                            method: 'POST',
                                            body: JSON.stringify({
                                                account: accounts[0], 
                                                message: signedMessage
                                            })
                                        });
            
            if (response.ok) {
                accountCache.set(accounts[0]);
            } else {
                console.log(response);
            }
        }
    };

    const disconnect = async () => {
        await modal.clearCachedProvider();
        const response = await fetch('/api/account/signout');
        if (response.ok) {
            accountCache.set(null);
        }
    };
</script>

<div class="relative min-h-screen select-none">
    <div class="absolute top-0 left-0 w-full h-full bg-brand-primary saturate-150 brightness-105">
        
    </div>
    <div id="baseParent" class="relative w-full h-full min-h-screen">
        <div class="sticky flex items-center top-0 z-50 font-mono px-12 py-5 bg-brand-primary drop-shadow-lg shadow-lg backdrop-blur-xl [@supports(backdrop-filter:blur(0px))]:bg-brand-primary/75">
            <span class="text-4xl text-brand-tertiary font-black">XYZ_MKT</span>
            <div class="inline-block ml-12 text-base text-brand-secondary flex-1 [&>a]:px-4">
                <a href="/"><span>Explore</span></a>
                <a href="/collections"><span>My_Collection</span></a>
                <a href="/listings"><span>My_Listings</span></a>
            </div>
            <div class="rounded-full bg-brand-tertiary/80 shadow-2xl px-4 py-2">
                {#if $accountCache == null}
                    <span class="text-brand-secondary cursor-pointer" on:click={connect}>Connect</span>
                {:else}
                    <span class="text-brand-secondary cursor-pointer" on:click={disconnect}>Disconnect</span>
                {/if}
            </div>
        </div>
        <div class="flex justify-center overflow-y-auto mx-32 py-4 h-full">
            <slot />
        </div>
    </div>
</div>