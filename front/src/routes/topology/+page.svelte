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
	});
</script>

<div
	class="p-3 flex justify-between items-center gap-2 h-full">
	<div bind:this={parentElement} class="justify-end w-full h-full ">
		<canvas bind:this={canvasElement} id="topology-canvas" />
	</div>

	<div class="flex justify-end h-full">
		<Sidebar {app} />
	</div>
</div>
