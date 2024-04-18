<script lang="ts">
	import { onMount, createEventDispatcher } from 'svelte';
	import FileExplorer from '../../../components/FileExplorer.svelte';
	import type { VolumeStatsResponse } from '$lib/types/VolumeStatsResponse';
	import type { VolumeRow } from '$lib/types/VolumeRow';
	import LineChartBytes from '../../../components/LineChartBytes.svelte';
	import { page } from '$app/stores';
	import { faArrowLeft, faTrash } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { formatBytes } from '$lib/FormatUtils';

	export let data;
	const id = data.name;
	const base64Name = data.base64Name;
	console.log(data);

	let statVolume: null | Array<[number, number]> = null;

	function formatStats(stats: VolumeRow[]): Array<[number, number]> {
		console.log('stats', stats);
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.dsk)];
		});
	}

	onMount(async () => {
		console.log('metaTitle idk', data.metaTitle);
		console.log('size', data.size);
		const statsUrl = '/volumes/' + $page.params.id + '/api/stats'
		console.log('stats', statsUrl)
		const response = await fetch(statsUrl);
		const statsRes = (await response.json()) as VolumeStatsResponse;
		statVolume = formatStats(statsRes.stats);
	});
</script>

<div class="flex gap-2 items-center mb-5">
	<a href="/volumes" class="btn btn-sm variant-soft">
		<Fa icon={faArrowLeft} fw class="mr-1" />
		Back to volumes
	</a>
</div>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
	{#if data.name !== data.mountpoint}
		<div>
			<span class="font-bold">Volume name : </span>
			{data.name}
		</div>
	{/if}
	{#if data.createdAt !== 'UNDEFINED'}
		<div>
			<span class="font-bold">Date of creation : </span>
			{data.createdAt}
		</div>
	{/if}
	<div>
		<span class="font-bold">Mountpoint location: </span>
		{data.mountpoint}
	</div>
	<div>
		<span class="font-bold">Volume size : </span>
		{formatBytes(data.size)}
	</div>
</div>

<br />
{#if statVolume}
	<LineChartBytes inputData={statVolume} />
{/if}

<div>
	<FileExplorer {base64Name}></FileExplorer>
</div>
