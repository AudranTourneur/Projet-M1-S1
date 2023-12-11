<script lang="ts">
	import { onMount } from 'svelte';

	let parentElement: HTMLDivElement;
	let canvasElement: HTMLCanvasElement;

	onMount(async () => {
        const { initApp } = await import('$lib/topology');
		initApp(canvasElement, parentElement);
	});
</script>

<div bind:this={parentElement} class="w-[80vw] h-[80vh]">
	<canvas bind:this={canvasElement} id="topology-canvas" />
</div>
