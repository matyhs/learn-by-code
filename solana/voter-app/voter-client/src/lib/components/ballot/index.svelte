<script>
    import { secretKey, programKey } from '$lib/scripts/stores.js';

    $: $programKey, shouldShowBallots()

    let showBallots = false;

    const programKeyExists = () => {
        return $programKey !== '';
    }

    const shouldShowBallots = () => {
        showBallots = programKeyExists();
    }

    const vote = async (value) => {
        await fetch('/api/ballot', {
            headers: {
                Authorization: $secretKey
            },
            method: 'POST',
            body: JSON.stringify({
                value,
                account: $programKey
            })
        })
        .then(response => response.json())
        .then(data => console.log(data))
        .catch(error => console.log(error));
    }

</script>

{#if showBallots}
<div class="w-full h-full">
    <div class="flex items-center">
        <span class="flex-1 text-white">Hello World</span>
        <button class="w-24 h-12 text-lg bg-yellow-custom-1 text-blue-custom-1 rounded-br-lg rounded-tl-lg" on:click={() => vote('Hello World')}>Vote</button>
    </div>
</div>
{/if}