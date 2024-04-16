<script lang="ts">
	import { faCheck, faCopy, faCube, faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import type { TopologyContainerPixi } from '$lib/TopologyContainerPixi';
	import copy from 'copy-to-clipboard';
	import LineChartBytes from '../../components/LineChartBytes.svelte';
	import type { ContainerStatisticsRow } from '$lib/types/ContainerStatisticsRow';
	import type { ContainerStatsResponse } from '$lib/types/ContainerStatsResponse';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	export let entity: TopologyContainerPixi;
	const data = entity.data.data;

	let isNameCopied = false;
	let isIdCopied = false;
	const copyToClipboardName = () => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(data.names[0].substring(1, data.names[0].length));
	};
	const copyToClipboardId = () => {
		isIdCopied = true;
		setTimeout(() => (isIdCopied = false), 1000);
		copy(data.id);
	};

	
	let inputData: null | Array<[number, number]> = null;

	function generateDayWiseTimeSeries(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.mem)];
		});
	}
	onMount(async () => {
		const response = await fetch('/containers/' + data.id + '/api/stats');
		const statsRes = (await response.json()) as ContainerStatsResponse;
		inputData = generateDayWiseTimeSeries(statsRes.stats);
	});
</script>

<div class="border border-red-500 flex gap-2">
	{#if data.iconUrl}
		<img src={data.iconUrl} alt={data.image} class="max-w-full max-h-[60px]" />
	{:else}
		<Fa icon={faCube} class="text-surface-400-500-token text-xl" />
	{/if}

	<div class="flex flex-col">
		<div class="copy-to-clipboard">
			<a href="/containers/{data.id}" class="btn variant-ghost p-2">
				<Tooltip tooltipText={`Container ID: ${data.id}`}>
					{data.id.substring(0, 12)}
				</Tooltip>
			</a>

			<button type="button" class="btn variant-soft" on:click={copyToClipboardId}>
				{#if isNameCopied}
					<Fa icon={faCheck} class="text-green-500" />
				{:else}
					<Fa icon={faCopy} />
				{/if}
			</button>
		</div>
		<div class="copy-to-clipboard">
			{#if data.names[0].length < 15}
				{data.names[0].substring(1, data.names[0].length)}
			{:else}
				<Tooltip tooltipText={data.names[0].substring(1, data.names[0].length)}>
					{data.names[0].substring(1, 12)}
				</Tooltip>
				<div class="hide-on-clipboard-hover">...</div>
			{/if}
			<div class="hide-on-clipboard-hover">...</div>
			<button type="button" class="btn variant-soft" on:click={copyToClipboardName}>
				{#if isNameCopied}
					<Fa icon={faCheck} class="text-green-500" />
				{:else}
					<Fa icon={faCopy} />
				{/if}
			</button>
		</div>
	</div>
</div>

<div>
	<span class="font-bold">Image ID : </span>
	{#if data.image.length < 15}
		{data.image.substring(1, data.image.length)}
	{:else}
		<Tooltip tooltipText={data.image.substring(1, data.image.length)}>
			{data.image.substring(1, 12)}
		</Tooltip>
		<span>...</span>
	{/if}
</div>
<div>
	<span class="font-bold">Status :</span>
	{data.status}
</div>
<div>
	<span class="font-bold">Networks : </span>
	{data.networks.join(', ')}
</div>
<div>
	<span class="font-bold">Volumes : </span>
	{#if data.volumes[0].length < 15}
		{data.volumes.join(', ')}
	{:else}
		<Tooltip tooltipText={data.volumes[0].substring(1, data.volumes[0].length)}>
			{data.volumes[0].substring(1, 12)}
		</Tooltip>
		<span>...</span>
	{/if}
	{data.volumes.join(', ')}
</div>
{#if inputData}
	<LineChartBytes {inputData} />
{/if}
