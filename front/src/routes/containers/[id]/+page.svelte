<script lang="ts">
	export let data;

	import { onMount } from 'svelte';

	onMount(() => {
		console.log('todo');
	});
</script>

{#each data.ports as port, i}
    <span>IP: {port.IP}  ||</span>
    <span>PrivatePort: {port.PrivatePort}  ||</span>
    <span>PublicPort: {port.PublicPort}  ||</span>
    <span>Type: {port.Type}</span>
    <br/>
{/each}