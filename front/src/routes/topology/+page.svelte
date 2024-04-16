<script lang="ts">
	import type { TopologyApp } from '$lib/TopologyApp.js';
	import { onMount } from 'svelte';
	import Sidebar from './Sidebar.svelte';

	let parentElement: HTMLDivElement;
	let canvasElement: HTMLCanvasElement;

	export let data;

	let app: TopologyApp;

	onMount(async () => {
		const { initApp } = await import('$lib/topology');
		app = initApp(canvasElement, parentElement, data);

		console.log('Topology data', data);
	});
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 flex justify-between items-center gap-2 h-full">
	<div bind:this={parentElement} class="justify-end w-full h-full ">
		<canvas bind:this={canvasElement} id="topology-canvas" />
	</div>

	<div class="w-[30vh] h-[72vh]">
		<Sidebar {app} />
	</div>
</div>
