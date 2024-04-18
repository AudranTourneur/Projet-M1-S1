<script lang="ts">
	import { onMount, createEventDispatcher } from 'svelte';
	import FileExplorer from '../../../components/FileExplorer.svelte';
	import { goto } from '$app/navigation';
	import PortsBox from '../../volumes/[id]/PortsBox.svelte';
	import type { VolumeStatsResponse } from '$lib/types/VolumeStatsResponse';
	import type { VolumeRow } from '$lib/types/VolumeRow';
	import LineChartBytes from '../../../components/LineChartBytes.svelte';
	import type { VolumeData } from '$lib/types/VolumeData';
	import { page } from '$app/stores';
	import { faArrowLeft, faTrash } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Chart from '../../../components/Chart.svelte';

	export let data;
	const id = data.name;
	console.log(data);

	const volume: VolumeData = data;

	let showModal = false;
	const dispatch = createEventDispatcher();

	let statVolume: null | Array<[number, number]> = null;

	async function handleDeleteVolume(volumeName: String) {
		showModal = true;
	}

	async function confirmDelete() {
		const response = await fetch(`/volumes/${id}/api/remove-volume`, {
			method: 'POST'
		});
		console.log(await response.text());
		showModal = false;
		dispatch('volumeDeleted', { name: data.name });
		goto('/volumes');
	}

	function cancelDelete() {
		showModal = false;
	}

	function generateDayWiseTimeSeries(stats: VolumeRow[]): Array<[number, number]> {
		console.log('stats', stats);
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.dsk)];
		});
	}

	onMount(async () => {
		console.log('metaTitle idk', data.metaTitle);
		console.log('size', data.size);
		const response = await fetch('/volumes/' + $page.params.id + '/api/stats');
		const statsRes = (await response.json()) as VolumeStatsResponse;
		statVolume = generateDayWiseTimeSeries(statsRes.stats);
	});
</script>

<div class="flex gap-2 items-center mb-5">
	<a href="/volumes" class="btn btn-sm variant-soft">
		<Fa icon={faArrowLeft} fw class="mr-1" />
		Back to volumes
	</a>
	<button class="btn btn-sm variant-ghost-error" on:click={() => handleDeleteVolume(data.name)}>
		<Fa icon={faTrash} fw />
		Delete volume
	</button>
	{#if showModal}
		<div class="fixed left-0 w-full h-full flex items-center justify-center">
			<div
				class="p-4 rounded shadow-md border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow">
				<p>Are you sure you want to delete this volume?</p>
				<div class="flex justify-between mt-4">
					<button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={confirmDelete}>Yes</button>
					<button class="bg-gray-500 text-white px-4 py-2 rounded" on:click={cancelDelete}>No</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
	<div>
		<span class="font-bold">Volume name : </span>
		{data.name}
	</div>
	<div>
		<span class="font-bold">Date of creation : </span>
		{data.createdAt}
	</div>
	<div>
		<span class="font-bold">Mountpoint location: </span>
		{data.mountpoint}
	</div>
	<div>
		<span class="font-bold">Volume size : </span>
		{data.size}
	</div>

	<!-- <div>
		<br />
		<PortsBox {volume}></PortsBox>
	</div> -->
</div>

<br />
{#if statVolume}
	<Chart containerID={data.name} typeChart=""/>
{/if}

<div>
	<FileExplorer {id}></FileExplorer>
</div>
