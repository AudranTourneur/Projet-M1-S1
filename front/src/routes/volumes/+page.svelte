<script lang="ts">
	import type { VolumeData } from '$lib/types/VolumeData';
	import VolumeBox from './VolumeBox.svelte';

	export let data;

	const volumes: VolumeData[] = data.volumes;
	console.log('aaaaaaa', volumes)
	let visibleVolumes = [...volumes];
	let search = '';

	function searchVolumes() {
		visibleVolumes = volumes.filter((volume) => volume.name.toLowerCase().includes(search.toLowerCase()));
	}
</script>

<h1 class="text-center text-4xl mb-5">Volumes</h1>
<div class="mx-auto max-w-xs mb-4">
	<input
		bind:value={search}
		type="text"
		placeholder="Search by name"
		class="input"
		on:input={searchVolumes} />
</div>
<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
	{#each visibleVolumes as volume}
		<VolumeBox {volume} />
	{/each}
</div>
