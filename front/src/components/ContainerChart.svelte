<script lang="ts">
	import { page } from '$app/stores';
	import type { ContainerStatisticsRow } from '$lib/types/ContainerStatisticsRow';
	import type { ContainerStatsResponse } from '$lib/types/ContainerStatsResponse';
	import { onMount } from 'svelte';
	import LineChartBytes from './LineChartBytes.svelte';

    export let containerID: string;

	let statData: null | Array<[number, number]> = null;

	function generateDayWiseTimeSeries(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.mem)];
		});
	}

	onMount(async () => {
		const response = await fetch('/containers/' + containerID + '/api/stats');
		const statsRes = (await response.json()) as ContainerStatsResponse;
		statData = generateDayWiseTimeSeries(statsRes.stats);
	});

    console.log(statData)
</script>


{#if statData}
	<LineChartBytes inputData={statData} />
{/if}