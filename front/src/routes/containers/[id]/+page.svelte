<script lang="ts">
	import { page } from '$app/stores';
	import type { ContainerStatsResponse } from '$lib/types/ContainerStatsResponse';
	import { onMount } from 'svelte';
	import LineChartBytes from '../../../components/LineChartBytes.svelte';
	import type { ContainerStatisticsRow } from '$lib/types/ContainerStatisticsRow.js';
	import PortsBox from './PortsBox.svelte';
	import { goto } from '$app/navigation';
	import Fa from 'svelte-fa';
	import { faArrowLeft, faTrash } from '@fortawesome/free-solid-svg-icons';
	import type { ContainerData } from '$lib/types/ContainerData';

	export let data;

	const container: ContainerData = data;

	let inputData: null | Array<[number, number]> = null;

	function generateDayWiseTimeSeries(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.mem)];
		});
	}

	onMount(async () => {
		const response = await fetch('/containers/' + $page.params.id + '/api/stats');
		const statsRes = (await response.json()) as ContainerStatsResponse;
		inputData = generateDayWiseTimeSeries(statsRes.stats);
	});

	async function removeContainer() {
		await fetch('/containers/' + $page.params.id + '/api/remove', {
			method: 'POST'
		});
		goto('/containers');
	}
</script>

<div class="flex gap-2 items-center mb-5">
	<a href="/containers" class="btn btn-sm variant-soft">
		<Fa icon={faArrowLeft} fw class="mr-1" />
		Back to containers
	</a>
	<button class="btn btn-sm variant-ghost-error" on:click={removeContainer}>
		<Fa icon={faTrash} fw class="mr-1" />
		Delete container
	</button>
</div>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
	<div>
		<span class="font-bold">ID : </span>
		{container.id}
	</div>
	<div>
		<span class="font-bold">Name : </span>
		{container.names.join(', ')}
	</div>
	<div>
		<span class="font-bold">Image ID : </span>
		{container.image}
	</div>
	<div>
		<span class="font-bold">Status :</span>
		{container.status}
	</div>
	<div>
		<span class="font-bold">Networks : </span>
		{container.networks.join(', ')}
	</div>
	<div>
		<span class="font-bold">Volumes : </span>
		{container.volumes.join(', ')}
	</div>
</div>

<h2 class="font-bold text-xl">Ports :</h2>
{#if container.ports.length > 0}
	<div>
		<div class="table-container">
			<table class="table table-compact">
				<thead>
					<tr>
						<th style="padding: 0.5rem">IP</th>
						<th style="padding: 0.5rem">Private Port</th>
						<th style="padding: 0.5rem">Public Port</th>
						<th style="padding: 0.5rem">Protocol</th>
					</tr>
				</thead>
				<tbody>
					{#each container.ports as port}
						<tr>
							<td>{port.ip}</td>
							<td>{port.privatePort}</td>
							<td>{port.publicPort}</td>
							<td>{port.type}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
{:else}
	<p class="italic">No ports exposed</p>
{/if}

{#if inputData}
	<LineChartBytes {inputData} />
{/if}

<PortsBox {container} />
