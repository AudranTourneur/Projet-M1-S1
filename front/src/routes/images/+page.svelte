<script lang="ts">
	import ImageBox from './ImageBox.svelte';

	export let data;

	const images = data.images;

	let visibleImages = [...images];

	let search = '';

	$: {
		visibleImages = images.filter(
			(img) =>
				img.id.toLowerCase().includes(search.toLowerCase()) ||
				img.tags.some((tag) => tag.toLowerCase().includes(search.toLowerCase()))
		);
	}
</script>

<h1 class="text-center text-4xl mb-5">Images</h1>
<div class="mx-auto max-w-xs">
	<input bind:value={search} type="text" placeholder="Search by name or ID" class="input mb-4" />
</div>

<div class="flex flex-col gap-4">
	{#each visibleImages as image}
		<ImageBox {image} />
	{/each}
</div>
