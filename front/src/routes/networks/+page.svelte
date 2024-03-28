<script lang="ts">
	export let data;
	console.log('data=', data);

	const networks = data.networks;
	console.log(networks)

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

{#each networks as network}
    {network.name} <br>
{/each}
