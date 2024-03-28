<script lang="ts">
	export let data;
	console.log('data=', data);

	const networks = data.networks;
	let search = '';
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

<h1 class="text-center text-4xl">Networks</h1>

{#each networks as network}
	{network.name} <br />
	<div class="space-y-5">
		<div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg overflow-auto">
			<div>
				<span>{network.name}</span>
			</div>
			<br />
			<div>
				<span>ID : {network.id}</span>
			</div>
			<br />
			<div>
				<span>Created :{network.created}</span>
			</div>
			<br />
			<div>
				Labels :
                <br />
                {#each Object.entries(network.labels) as [str1, str2]}
                {str1}    :    {str2}
                <br />
                {/each}
			</div>
			<br />
			<div>
                Ipam Config :
				{#each network.ipamConfig as config}
					<div>
						<p>Subnet: {config.subnet}</p>
						<p>IP Range: {config.ipRange}</p>
						<p>Gateway: {config.gateway}</p>
						{#if config.auxAddresses}
							<p>Aux Addresses:</p>
							<ul>
								{#each Object.entries(config.auxAddresses) as [key, value]}
									<li>{key}: {value}</li>
								{/each}
							</ul>
						{/if}
					</div>
				{/each}
			</div>
            <br />    
            <div>
                Containers :
                {#each Object.entries(network.containers) as [name,other]}
                <div>
                    <p>Name: {other.name}</p>
                    <p>Endpoint Id: {other.endpointId}</p>
                    <p>Mac Address: {other.macAddress}</p>
                    <p>IPv4 Address: {other.ipv4Address}</p>
                    <p>IPv6 Address: {other.ipv6Address}</p>
                    <br />
                </div>
                {/each}
            </div>
            <br />
			<div class="flex justify-end">
				<a href="/network/{network.id}">
					<button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info </button>
				</a>
			</div>
		</div>
	</div>
{/each}
