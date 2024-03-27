<script lang="ts">
	import type { TopologyApp } from '$lib/TopologyApp.js';
	import { onMount } from 'svelte';

	let parentElement: HTMLDivElement;
	let canvasElement: HTMLCanvasElement;

	export let data;

	let app: TopologyApp;

	onMount(async () => {
		const { initApp } = await import('$lib/topology');
		app = initApp(canvasElement, parentElement, data);

		console.log('Topology data', data);
	});

	async function save() {
		console.log('save')
		const res = await fetch('/topology/api/save', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify(app.getSaveData()),
		})
		console.log('RES', await res.text())
	}
</script>

<div class="flex justify-end items-center">
	<button type="button" class="btn variant-filled" on:click={save}>Save</button>
</div>

<div bind:this={parentElement} class="w-[80vw] h-[80vh]">
	<canvas bind:this={canvasElement} id="topology-canvas" />
</div>
