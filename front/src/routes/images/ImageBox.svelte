<script lang="ts">
	import { formatBytes, formatCreatedDate } from '$lib/FormatUtils.js';
	import type { ImageData } from '$lib/types/ImageData.js';

	export let image: ImageData;
	console.log(image);

	const deleteVolume = () => {
		// Implement the logic to delete the volume with the given index
		console.log(`Deleting volume with id ${image.id}`);
	};
</script>

<div class="space-y-5">
	<div class="relative p-3 m-2 shadow rounded-lg overflow-auto">
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
				<button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info</button>
			</a>
			<button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={deleteVolume}>Delete </button>
		</div>
	</div>
</div>
