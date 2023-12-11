<script lang="ts">
	import { onMount } from 'svelte';
	import { initApp } from '$lib/topology';

    let parentElement: HTMLDivElement;
	let canvasElement: HTMLCanvasElement;

	onMount(() => {
		initApp(canvasElement, parentElement);
	});
</script>

    <div bind:this={parentElement}>
<canvas bind:this={canvasElement} id="topology-canvas" />
        </div>
