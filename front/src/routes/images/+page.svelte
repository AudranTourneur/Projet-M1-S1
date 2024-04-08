<script lang="ts">
	import type { ImageData } from '$lib/types/ImageData.js';
	import ImageBox from './ImageBox.svelte';
	export let data;

	let images : ImageData[] = data.images;
	let visibleImages = [...images];
	let search = '';

	$: {
		visibleImages = images.filter(
			(img) =>
				img.id.toLowerCase().includes(search.toLowerCase()) ||
				img.tags.some((tag) => tag.toLowerCase().includes(search.toLowerCase()))
		);
	}

	const refetchImages = async () => {
		await fetch("/images/api/list")
			.then(response => response.json())
			.then(data => {
				images = data.images;
			});
	}

</script>

<h1 class="text-center text-4xl mb-5">Images</h1>
<div class="mx-auto max-w-xs">
	<input bind:value={search} type="text" placeholder="Search by name or ID" class="input mb-4" />
</div>

<div class="flex flex-col gap-4">
	{#each visibleImages as image}
		<ImageBox {image} refresh={refetchImages} />
	{/each}
</div>
