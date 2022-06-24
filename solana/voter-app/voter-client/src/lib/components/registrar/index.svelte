<script>
    import { Keypair } from '@solana/web3.js';
    import { onMount } from 'svelte';
    import { programKey, secretKey } from '$lib/scripts/stores.js';
    import { Buffer } from 'buffer';

    $: $programKey, hideRegister();

    let showRegister = true;

    onMount(() => {
        let base64Keypair = window.localStorage.getItem('secretKey');

        if (!base64Keypair) {
            const keypair = new Keypair();
            base64Keypair = Buffer.from(keypair.secretKey).toString('base64');
            window.localStorage.setItem('secretKey', base64Keypair);
        }
        
        secretKey.set(base64Keypair);
    });

    const register = async() => {
        await fetch('/api/account', {
            headers: {
                Authorization: $secretKey
            },
            method: 'POST'
        })
        .then(response => response.json())
        .then(data => programKey.set(data.ballot))
        .catch(error => {
            console.log(error);
        });
    };

    const hideRegister = () => {
        showRegister = false;
    }
</script>

{#if showRegister}
<form class="w-24 h-12" method="post" action="/api/account" on:submit|preventDefault={register}>
    <button class="text-lg bg-yellow-custom-1 text-blue-custom-1 rounded-bl-lg rounded-tr-lg w-full h-full">Register</button>
</form>
{:else}
<span class="text-white">Welcome back: {$programKey}</span>
{/if}
