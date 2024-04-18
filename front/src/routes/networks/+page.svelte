<script lang="ts">
    import NetworkBox from './NetworkBox.svelte';
    export let data;
    export let refresh: () => void;
    let networks = data.networks;
    let search = '';

    let visibleNetworks = [...networks];

    function startNetwork(index: number) {
        // todo
    }

    function stopNetwork(index: number) {
        // todo
    }

    function restartNetwork(index: number) {
        // todo
    }

    $: visibleNetworks = networks.filter(network => {
        const searchString = search.toLowerCase();
        return network.name.toLowerCase().includes(searchString) || network.id.includes(searchString);
    });

    const refetchNetworks = async () => {
        await fetch("/networks/api/list")
            .then(response => response.json())
            .then(data => {
                networks = data.networks;
            });
    }
</script>

<h1 class="text-center text-4xl mb-5">
    Networks
</h1>

<div class="mx-auto max-w-xs mb-5">
    <input bind:value={search} type="text" placeholder="Search by ID or Network" class="input"/>
</div>

<div class="flex flex-col gap-4">
    {#each visibleNetworks as network}
        <NetworkBox {network} refresh={refetchNetworks} />
    {/each}
</div>



