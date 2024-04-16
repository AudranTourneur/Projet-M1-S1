<script lang="ts">
    import { onMount, createEventDispatcher } from 'svelte';
	import FileExplorer from '../FileExplorer.svelte';
	import { goto } from '$app/navigation';
	import PortsBox from '../../volumes/[id]/PortsBox.svelte';
    import type {VolumeStatsResponse} from '$lib/types/VolumeStatsResponse';
    import type { VolumeRow } from '$lib/types/VolumeRow';
	import LineChartBytes from '../../../components/LineChartBytes.svelte';
	import type { VolumeData } from '$lib/types/VolumeData';
	import { page } from '$app/stores';

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
		console.log("stats", stats);
        return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.dsk)];
		});
	}

	onMount(async () => {
        console.log("metaTitle idk", data.metaTitle)
        console.log("size", data.size)
        const response = await fetch('/volumes/' + $page.params.id + '/api/stats');
        const statsRes = (await response.json()) as VolumeStatsResponse;
        statVolume = generateDayWiseTimeSeries(statsRes.stats);
	});
</script>


{#if statVolume}
	<LineChartBytes inputData={statVolume} />
{/if}

<div>
	<br />
	{data.name}
	<br />
	{data.createdAt}
	<br />
	{data.mountpoint}
	<br />
	{data.size}
	<br />
	<button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => handleDeleteVolume(data.name)}>
		Delete
	</button>

	{#if showModal}
		<div class="fixed top-0 left-0 w-full h-full flex items-center justify-center bg-gray-800 bg-opacity-50">
			<div class="bg-black p-4 rounded shadow-md">
				<p>Are you sure you want to delete this volume?</p>
				<div class="flex justify-between mt-4">
					<button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={confirmDelete}>Yes</button>
					<button class="bg-gray-500 text-white px-4 py-2 rounded" on:click={cancelDelete}>No</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<br />
<div>
	<h2>Filesystem</h2>
	<FileExplorer {id}></FileExplorer>
</div>

<div>
	<br />
	<PortsBox {volume}></PortsBox>
</div>
