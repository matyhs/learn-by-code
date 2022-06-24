<script>
    import Registrar from '$lib/components/registrar/index.svelte';
    import Verifier from '$lib/components/verifier/index.svelte';
    import Ballot from '$lib/components/ballot/index.svelte';
</script>

<Verifier>
    <div class="flex flex-col w-screen h-screen bg-blue-custom-1">
        <Registrar />
        <div class="flex-1">
            <Ballot />
        </div>
    </div>
</Verifier>

