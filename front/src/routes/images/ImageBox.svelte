<script lang="ts">
	import { formatBytes, formatCreatedDate } from '$lib/FormatUtils.js';
	import type { ImageData } from '$lib/types/ImageData.js';
	import { faCircleNotch, faEllipsisVertical, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { Fa } from 'svelte-fa';

	export let image: ImageData;

	let isLoadingRemove = false;
	const deleteVolume = async () => {
		isLoadingRemove = true;
		await fetch(`/images/${image.id}/api/remove`, {
			method: 'POST'
		});
		isLoadingRemove = false;
		// refresh();
	};
</script>

<div
	class="flex justify-between items-center gap-2 p-3 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border-surface-300-600-token">
	<img src={image.iconUrl} alt="icon" class="w-8 h-8 rounded-full" />
	<div class="flex flex-col">
		<div>
			<span>{image.tags}</span>
		</div>
		<br />
		<div>
			<span>ID : {image.id.substring(0, 19)}...</span>
		</div>
	</div>
	<div class="flex flex-col">
		<div>
			<span>Created :{formatCreatedDate(image.created)}</span>
		</div>
		<br />
		<div>
			Size :{formatBytes(image.size)}
		</div>
	</div>

	<div class="flex gap-1">
		<button class="btn variant-filled-error p-2" on:click={deleteVolume} disabled={isLoadingRemove}>
			<Fa icon={!isLoadingRemove ? faTrash : faCircleNotch} spin={isLoadingRemove} fw />
		</button>
		<a href="/images/{image.id}" class="btn variant-ghost p-2">
			<Fa icon={faEllipsisVertical} fw />
		</a>
	</div>
</div>
