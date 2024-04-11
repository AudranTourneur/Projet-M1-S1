<script lang="ts">
	import type { TopologyApp } from '$lib/TopologyApp.js';
	import { TopologyContainerPixi } from '$lib/TopologyContainerPixi.js';
	import { currentlySelectedEntity } from '$lib/TopologyStore.js';
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


		console.log($currentlySelectedEntity)
	}


</script>

<div class='flex'>
<div bind:this={parentElement} class="flex justify-end w-[80vw] h-[72vh]">
	<canvas bind:this={canvasElement} id="topology-canvas" />
</div>

<div class="flex items-center">
	{#if $currentlySelectedEntity?.entity instanceof TopologyContainerPixi }
	{$currentlySelectedEntity?.entity.data}
	{/if}

	<button type="button" class="btn justify-end variant-filled" on:click={save}>Save</button>
</div>
</div>