<script lang="ts">
	export let data;
	console.log('data=', data);

	const networks = data.networks;
	let search = ''

    let visibleNetworks = [...networks];
    const images = data.images;

	// let response = JSON.parse(data)
	// console.log(response);

	function startNetwork(index: number) {
		// todo
	}

	function stopNetwork(index: number) {
		// todo
	}

	function restartNetwork(index: number) {
		// todo
	}
</script>


<!-- <h1 class="text-center text-4xl">Networks</h1>
<br />
<div class="mx-auto max-w-xs">
	<input bind:value={search} type="text" placeholder="Search by name or ID" class="input mb-4" /> ### à modifier
</div>
--> 
{#each networks as network}
    <br />
    <div class="flex justify-between items-start gap-2 p-3 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border-surface-300-600-token">
        <div class="flex flex-col">
            <div>
                <span><u>{network.name ? network.name : 'none'}</u></span>
            </div>
            <br />
            <div>
                <span><b>ID :</b> {network.id ? network.id.substring(0, 19) + '...' : 'none'}</span>
            </div>
            <br />
            <div>
                <span><b>Created :</b> {network.created ? network.created : 'none'}</span>
            </div>
        </div>
        <div class="flex flex-col">
            <div>
                <b>Labels :</b>
                <br />
                {#if network.labels}
                    {#each Object.entries(network.labels) as [str1, str2]}
                        {str1} : {str2}
                        <br />
                    {/each}
                {:else}
                    none
                {/if}
            </div>
        </div>
        <div class="flex flex-col">
            <div>
                <b>Ipam Config :</b>
                {#if network.ipamConfig}
                    {#each network.ipamConfig as config}
                        <p>Subnet: {config.subnet ? config.subnet : 'none'}</p>
                        <p>IP Range: {config.ipRange ? config.ipRange : 'none'}</p>
                        <p>Gateway: {config.gateway ? config.gateway : 'none'}</p>
                        {#if config.auxAddresses}
                            <p>Aux Addresses:</p>
                            <ul>
                                {#each Object.entries(config.auxAddresses) as [key, value]}
                                    <li>{key}: {value}</li>
                                {/each}
                            </ul>
                        {/if}
                    {/each}
                {:else}
                    none
                {/if}
            </div>
        </div>
        <div class="flex flex-col">
            <div>
                <b>Containers :</b>
                {#if network.containers}
                    <div class="mt-5 grid grid-cols-3 gap-4">
                        {#each Object.entries(network.containers) as [id, c]}
                            <div class="flex items-center gap-4 border border-1 p-2 px-4 rounded-lg">
                                <div class="flex flex-col">
                                    <a href="/containers/{id}" class="text-xl font-semibold hover:text-surface-500">{c.name}</a>
                                    <span class="text-surface-600-300-token">{c.ipv4Address}</span>
                                    <span class="text-surface-600-300-token">{c.macAddress}</span>
                                </div>
                                <br />
                            </div>
                        {/each}
                    </div>
                {:else}
                    none
                {/if}
            </div>
        </div>
        <div class="flex justify-end">
            <a href="/networks/{network.id}">
                <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info </button>
            </a>
        </div>
    </div>
{/each}
