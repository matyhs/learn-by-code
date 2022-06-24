<script>
    import { onMount } from 'svelte';
    export let solana;

    onMount(async () => {
        console.log(solana.payor._keypair);
    });
</script>

<p>Transaction fee: {solana.fees}</p>
<p>Current balance: {solana.balance}</p>
<form method="post" action="/">
    <button type="submit">Click me!</button>
</form>
