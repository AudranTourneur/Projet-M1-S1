<div class="text-center font-bold text-xl py-3">
    Network
</div>

<!--GIT TEST FROM XAVIER'S SERVER-->

<div class="flex items-center gap-4 p-3">
    <div class="flex-grow"></div>
    <input type="text" placeholder="Search by ID or Network" class="input" bind:value={search} />
</div>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-4">
    {#each visibleNetworks as network}
        <div class="p-3 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border-surface-300-600-token mt-4">
            <div class="font-bold text-lg mb-2">{network.name ? network.name : 'none'}</div>
            <div class="flex justify-between items-start gap-2">
                {#if network.id}
                    <div class="flex flex-col">
                        <div class="text-surface-600-300-token">
                            <b>ID :</b>
                            <span class="cursor-pointer" title="{network.id}" on:click={() => navigator.clipboard.writeText(network.id)}>
                                {network.id.substring(0, 8)}...
                            </span>
                        </div>
                    </div>
                {/if}
                {#if network.created}
                    <div class="flex flex-col">
                        <div class="text-surface-600-300-token">
                            <b>Created :</b> 
                            <span class="cursor-pointer" title="{network.created}" on:click={() => navigator.clipboard.writeText(network.created)}>
                                {network.created.substring(0, 10)}...
                            </span>
                        </div>
                    </div>
                {/if}
                {#if network.labels}
                    <div class="flex flex-col">
                        <div>
                            <b>Labels :</b>
                            <br />
                            {#each Object.entries(network.labels) as [str1, str2]}
                                <div>{str1} : {str2}</div>
                            {/each}
                        </div>
                    </div>
                {/if}
                {#if network.ipamConfig}
                    <div class="flex flex-col">
                        <div>
                            <b>Ipam Config :</b>
                            {#each network.ipamConfig as config}
                                <div>Subnet: {config.subnet ? config.subnet : 'none'}</div>
                                <div>IP Range: {config.ipRange ? config.ipRange : 'none'}</div>
                                <div>Gateway: {config.gateway ? config.gateway : 'none'}</div>
                                {#if config.auxAddresses}
                                    <div>Aux Addresses:</div>
                                    <ul>
                                        {#each Object.entries(config.auxAddresses) as [key, value]}
                                            <li>{key}: {value}</li>
                                        {/each}
                                    </ul>
                                {/if}
                            {/each}
                        </div>
                    </div>
                {/if}
            </div>
            <!--<div class="mt-3 grid grid-cols-5 gap-4">
                {#if network.containers}
                    {#each Object.entries(network.containers) as [id, c]}
                        <div class="flex items-center gap-4 border border-1 p-2 px-4 rounded-lg">
                            <div class="flex flex-col">
                                <a href="/containers/{id}" class="text-xl font-semibold hover:text-surface-500 whitespace-nowrap overflow-hidden overflow-ellipsis">{c.name}</a>
                                <span class="text-surface-600-300-token">{c.ipv4Address ? c.ipv4Address : 'none'}</span>
                                <span class="text-surface-600-300-token">{c.macAddress ? c.macAddress : 'none'}</span>
                            </div>
                        </div>
                    {/each}
                {:else}
                    <div>none</div>
                {/if}
            </div>-->
            <div class="flex justify-end mt-3">
                <a href="/networks/{network.id}">
                    <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info </button>
                </a>
            </div>
        </div>
    {/each}
</div>



<script lang="ts">
    import NetworkBox from './NetworkBox.svelte';
    export let data;
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

</script>

<h1 class="text-center text-4xl mb-5">
    Networks
</h1>

<div class="mx-auto max-w-xs mb-5">
    <input bind:value={search} type="text" placeholder="Search by ID or Network" class="input"/>
</div>

<div class="flex flex-col gap-4">
    {#each visibleNetworks as network}
        <NetworkBox {network}/>
    {/each}
</div>



