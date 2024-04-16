<script lang="ts">
	import type { VolumeData } from '$lib/types/VolumeData';
	import { faDownload, faEllipsisVertical } from '@fortawesome/free-solid-svg-icons';
	import { Fa } from 'svelte-fa';

	export let volume: VolumeData;

	const downloadVolume = () => {
		// implémenter la fonction adéquate
		console.log(`Downloading volume with index ${volume.name}`);
	};
</script>

<div class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4 flex justify-between items-center gap-4">
    <div class="flex flex-col gap-4">
        <div class="text-ellipsis overflow-hidden">
            <span class="font-bold">Name:</span>
            {volume.name}
        </div>
        <div class="text-ellipsis overflow-hidden">
            <span class="font-bold">Created at:</span>
            {new Date(volume.createdAt).toLocaleString()}
        </div>
        <div class="text-ellipsis overflow-hidden">
            <span class="font-bold">Mountpoint:</span>
            {volume.mountpoint}
        </div>
        <div class="text-ellipsis overflow-hidden">
            <span class="font-bold">Size:</span>
            {volume.size}
        </div>
    </div>
    <div class="flex items-center gap-4">
        <button class="btn variant-filled-primary" on:click={downloadVolume}>
            <Fa icon={faDownload} fw />
            Download
        </button>
        <a href="/volumes/{volume.name}" class="btn variant-ghost p-2">
            <Fa icon={faEllipsisVertical} fw />
        </a>
    </div>
</div>




