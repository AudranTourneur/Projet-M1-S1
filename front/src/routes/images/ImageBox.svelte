<script lang="ts">
	import { formatBytes } from '$lib/FormatUtils.js';
	import type { ImageData } from '$lib/types/ImageData.js';
	import { faCircleNotch, faEllipsisVertical, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { Fa } from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';

	export let image: ImageData;
	export let refresh: () => void;

	let isLoadingRemove = false;
	const deleteImage = async () => {
		isLoadingRemove = true;
		await fetch(`/images/${image.id}/api/remove`, {
			method: 'POST'
		});
		isLoadingRemove = false;
		refresh();
	};
</script>

<div
	class="flex justify-between items-center gap-2 p-3 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border-surface-300-600-token">
	<div class="flex items-center gap-2">
		<img src={image.iconUrl} alt="icon" class="w-8 h-8 rounded-full" />
		<div class="flex flex-col">
			<div class="font-bold text-lg">
				{image.tags}
			</div>
			<Tooltip tooltipText={`ID : ${image.id}`}>
				{image.id.substring(0, 19)}...
			</Tooltip>
		</div>
	</div>
	<div class="flex flex-col">
		<div>
			<span class="font-bold">Created :</span>
			{new Date(image.created).toLocaleString()}
		</div>
		<div>
			<span class="font-bold">Size :</span>
			{formatBytes(image.size)}
		</div>
	</div>

	<div class="flex gap-1">
		<button class="btn variant-filled-error p-2" on:click={deleteImage} disabled={isLoadingRemove}>
			<Fa icon={!isLoadingRemove ? faTrash : faCircleNotch} spin={isLoadingRemove} fw />
		</button>
		<a href="/images/{image.id}" class="btn variant-ghost p-2">
			<Fa icon={faEllipsisVertical} fw />
		</a>
	</div>
</div>
