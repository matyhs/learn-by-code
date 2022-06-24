<script>
    import { secretKey, programKey } from '$lib/scripts/stores.js';

    $: $secretKey, verifyProgramAccount()

    const verifyProgramAccount = async () => {
        if ($secretKey !== '') {
            await fetch('/api/account', {
                headers: {
                    Authorization: $secretKey
                },
                method: 'GET'
            })
            .then(response => response.json())
            .then(data => programKey.set(data.account))
            .catch(error => console.log(error));
        }
    };
</script>

<slot />