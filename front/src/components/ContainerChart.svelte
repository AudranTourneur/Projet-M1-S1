<script lang="ts">
	import { page } from '$app/stores';
	import type { ContainerStatisticsRow } from '$lib/types/ContainerStatisticsRow';
	import type { ContainerStatsResponse } from '$lib/types/ContainerStatsResponse';
	import { onMount } from 'svelte';
	import LineChartBytes from './LineChartBytes.svelte';

	export let containerID: string;
	export let typeChart: string;

	let statData: null | Array<[number, number]> = null;
	let secondstatData: null | Array<[number, number]> = null;

	
	function generateTimeMem(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.mem)];
		});
	}

	function generateTimeCpu(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.cpu)];
		});
	}

	function generateTimeIoRead(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.ioRead)];
		});
	}

	function generateTimeIoWrite(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.ioWrite)];
		});
	}

	function generateTimeNetUp(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.netUp)];
		});
	}

	function generateTimeNetDown(stats: ContainerStatisticsRow[]): Array<[number, number]> {
		return stats.map((obj) => {
			return [Number(obj.ts) * 1000, Number(obj.netDown)];
		});
	}

	onMount(async () => {
		const response = await fetch('/containers/' + containerID + '/api/stats');
		const statsRes = (await response.json()) as ContainerStatsResponse;
		if (typeChart === 'Mem') {
			statData = generateTimeMem(statsRes.stats);
		} else if (typeChart === 'Cpu') {
			statData = generateTimeCpu(statsRes.stats);
		} else if (typeChart === 'Io') {
			statData = generateTimeIoRead(statsRes.stats);
			secondstatData = generateTimeIoWrite(statsRes.stats);
		} else if (typeChart === 'Net') {
			statData = generateTimeNetUp(statsRes.stats);
			secondstatData = generateTimeNetDown(statsRes.stats);
		}
	});
</script>

{#if statData}
	<LineChartBytes inputData={statData} secondinputData={secondstatData}/>
{/if}
