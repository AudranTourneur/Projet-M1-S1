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

<div class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4 flex flex-col sm:flex-row justify-between items-center gap-2">
    <div class="overflow-hidden flex flex-col gap-2">
        <div class="text-ellipsis overflow-hidden">
            <span class="font-bold">{volume.name}</span>  
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
    <div class="flex justify-end items-center gap-2">
        <button class="btn variant-filled-primary" on:click={downloadVolume}>
            <Fa icon={faDownload} fw />
        </button>
        <a href="/volumes/{volume.name}" class="btn variant-ghost p-2">
            <Fa icon={faEllipsisVertical} fw />
        </a>
    </div>
</div>

