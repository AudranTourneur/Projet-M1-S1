<script lang="ts">
	import { page } from '$app/stores';
	import type { ContainerStatsResponse } from '$lib/types/ContainerStatsResponse';
	import { onMount } from 'svelte';
	import LineChartBytes from '../../../components/LineChartBytes.svelte';
	import type { ContainerStatisticsRow } from '$lib/types/ContainerStatisticsRow.js';
	import PortsBox from './PortsBox.svelte';
	import { goto } from '$app/navigation';
	import Fa from 'svelte-fa';
	import { faArrowLeft, faCircleNotch, faPlay, faStop, faTrash } from '@fortawesome/free-solid-svg-icons';
	import type { ContainerData } from '$lib/types/ContainerData';
	import ContainerStatusIcon from '../ContainerStatusIcon.svelte';
	import { getContainerActionsFromStatus } from '../getContainerActionsFromStatus';
	import ContainerChart from '../../../components/ContainerChart.svelte';

	export let data;

	let container: ContainerData = data;

	const refetchContainer = async () => {
		await fetch("/containers/" + container.id + "/api/fetch")
			.then(response => response.json())
			.then(data => {
				container = data;
			});
	}

	let statData: null | Array<[number, number]> = null;

	let statusIcon: 'running' | 'stopped' | 'paused', canBeStarted: boolean, canBeStopped: boolean;
  $: {
    // use the function getContainerActionsFromStatus(container.status);
		const actions = getContainerActionsFromStatus(container.status);
		statusIcon = actions.statusIcon;
		canBeStarted = actions.canBeStarted;
		canBeStopped = actions.canBeStopped;
  }

	function generateDayWiseTimeSeries(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.mem)];
		});
	}

	onMount(async () => {
		const response = await fetch('/containers/' + $page.params.id + '/api/stats');
		const statsRes = (await response.json()) as ContainerStatsResponse;
		statData = generateDayWiseTimeSeries(statsRes.stats);
	});

	console.log(statData)


	let isLoadingStart = false;
	let isLoadingStop = false;
	let isLoadingRemove = false;

	const startContainer = async () => {
		isLoadingStart = true;
		await fetch(`/containers/${container.id}/api/start`, {
			method: 'POST'
		});
		isLoadingStart = false;
		refetchContainer();
	};

	const stopContainer = async () => {
		isLoadingStop = true;
		await fetch(`/containers/${container.id}/api/stop`, {
			method: 'POST'
		});
		isLoadingStop = false;
		refetchContainer();
	};

	const removeContainer = async () => {
		isLoadingRemove = true;
		await fetch('/containers/' + $page.params.id + '/api/remove', {
			method: 'POST'
		});
		isLoadingRemove = false;
		goto('/containers');
	};
</script>

<div class="flex gap-2 items-center mb-5">
	<a href="/containers" class="btn btn-sm variant-soft">
		<Fa icon={faArrowLeft} fw class="mr-1" />
		Back to containers
	</a>
	<ContainerStatusIcon status={statusIcon} statusString={container.status} />
	<button
		class="btn btn-sm variant-ghost-success"
		disabled={!canBeStarted || isLoadingStart}
		on:click={startContainer}>
		<Fa icon={!isLoadingStart ? faPlay : faCircleNotch} spin={isLoadingStart} fw />
		Start
	</button>
	<button
		class="btn btn-sm variant-ghost-error"
		disabled={!canBeStopped || isLoadingStop}
		on:click={stopContainer}>
		<Fa icon={!isLoadingStop ? faStop : faCircleNotch} spin={isLoadingStop} fw />
		Stop
	</button>
	<button class="btn btn-sm variant-ghost-error"
		disabled={isLoadingRemove}
					on:click={removeContainer}>
		<Fa icon={!isLoadingRemove ? faTrash : faCircleNotch} spin={isLoadingRemove} fw />
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


<ContainerChart containerID={container.id}/>

<PortsBox {container} />
