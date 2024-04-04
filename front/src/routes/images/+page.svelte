<script lang="ts">
	import { formatBytes, formatCreatedDate } from '$lib/FormatUtils';

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

	function deleteVolume(index: number): void {
		// Implement the logic to delete the volume with the given index
		console.log(`Deleting volume with index ${index}`);
	}
</script>

<div class="flex items-center justify-center">
	<form action="#" method="get" class="p-4 flex flex-col justify-center items-center gap-2">
		<h2 class="font-bold text-lg">Search by name or ID</h2>
		<input
			bind:value={search}
			type="text"
			placeholder="Search..."
			class=" p-2 border rounded-l"
		/>
	</form>
</div>

<h1 class="text-center text-4xl">Images</h1>
<ul>
	{#each visibleImages as image, i}
		<div class="space-y-5">
			<div class="relative p-3 m-2  shadow rounded-lg overflow-auto">
				<div>
					<span>{image.tags}</span>
				</div>
				<br />
				<div>
					<span>ID : {image.id}</span>
				</div>
				<br />
				<div>
					<span>Created :{formatCreatedDate(image.created)}</span>
				</div>
				<br />
				<div>
					Size :{formatBytes(image.size)}
				</div>

				<div class="flex justify-end">
					<a href="/images/{image.id}">
						<button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info </button>
					</a>
					<button
						class="bg-red-500 text-white px-4 py-2 rounded mr-2"
						on:click={() => deleteVolume(i)}>Delete</button
					>
				</div>
			</div>
		</div>
	{/each}
</ul>
