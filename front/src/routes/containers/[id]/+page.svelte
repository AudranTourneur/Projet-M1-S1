<script lang="ts">
	import { page } from '$app/stores';
	import type { ContainerStatsResponse } from '$lib/types/ContainerStatsResponse';
	import { onMount } from 'svelte';
	import LineChartBytes from '../../../components/LineChartBytes.svelte';
	import type { ContainerStatisticsRow } from '$lib/types/ContainerStatisticsRow.js';
	import PortsBox from './PortsBox.svelte';
	import { goto } from '$app/navigation';

	export let data;

	const container = data;

	let inputData: null | Array<[number, number]> = null;

	function generateDayWiseTimeSeries(
		stats: ContainerStatisticsRow[]
	): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.mem)];
		});
	}

	onMount(async () => {
		const response = await fetch('/containers/' + $page.params.id + '/api/stats');
		const statsRes = (await response.json()) as ContainerStatsResponse;
		console.log(statsRes);

		inputData = generateDayWiseTimeSeries(statsRes.stats)
	});

	async function removeContainer() {
		const serverResponse = await fetch('/containers/' + $page.params.id + '/api/remove', {
			method: 'POST'
		});
		console.log(serverResponse);
		goto('/containers');
	}
</script>

{#if inputData}
	<LineChartBytes {inputData} />
{/if}

<div class="border-token border-surface-300-600-token rounded-container-token p-4 mb-4">
	<div class="flex justify-between items-center mb-2">
		<span class="font-bold">ID:</span>
		<span>{data.id}</span>
	</div>
	<div class="flex justify-between items-center mb-2">
		<span class="font-bold">Name:</span>
		<span>{data.names}</span>
	</div>
	<div class="flex justify-between items-center mb-2">
		<span class="font-bold">Image ID :</span>
		<span>{data.image}</span>
	</div>
	<div class="flex justify-between items-center mb-2">
		<span class="font-bold">Status :</span>
		<span>{data.status}</span>
	</div>
	<div class="flex justify-between items-center mb-2">
		<span class="font-bold">Date of creation :</span>
		<span>{data.networks}</span>
	</div>
	<div class="flex justify-between items-center mb-2">
		{#each data.ports as port}
			<span>IP: {port.ip}</span>
			<span>PrivatePort: {port.privatePort}</span>
			<span>PublicPort: {port.publicPort}</span>
			<span>Type: {port.type}</span>
		{/each}
	</div>
	<div class="flex justify-between items-center mb-2">
		<span class="font-bold">Ports used :</span>
		<span>{data.volumes}</span>
	</div>
</div>

<button class="btn variant-ghost-error" on:click={removeContainer}>Delete container</button>

<div>
	<br>
	<PortsBox {container}></PortsBox>
</div>