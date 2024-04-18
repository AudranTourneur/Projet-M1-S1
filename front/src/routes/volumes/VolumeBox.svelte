<script lang="ts">
	import type { VolumeData } from '$lib/types/VolumeData';
	import {
		faCheck,
		faCopy,
		faDatabase,
		faDownload,
		faEllipsisVertical,
		faFolderOpen,
		faFolderPlus
	} from '@fortawesome/free-solid-svg-icons';
	import { Fa } from 'svelte-fa';
	import copy from 'copy-to-clipboard';

	export let volume: VolumeData;

	const downloadVolume = () => {
		// implémenter la fonction adéquate
		console.log(`Downloading volume with index ${volume.name}`);
	};

	let isNameCopied = false;

	const copyToClipboardName = () => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(volume.name);
	};
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3">
	<div class="text-lg copy-to-clipboard flex max-w-full mb-5 whitespace-nowrap">
		<div class="font-bold mr-1">Name:</div>
		<div class="overflow-hidden text-ellipsis">{volume.name}</div>
		<button type="button" class="btn variant-soft" on:click={copyToClipboardName}>
			{#if isNameCopied}
				<Fa icon={faCheck} class="text-green-500" />
			{:else}
				<Fa icon={faCopy} />
			{/if}
		</button>
	</div>
	<div class="flex flex-col sm:flex-row justify-between items-center gap-4">
		<div class="overflow-hidden max-w-full flex flex-col gap-2 w-full">
			<div class="text-ellipsis overflow-hidden flex items-center gap-2 whitespace-nowrap">
				<Fa icon={faFolderPlus} fw />
				<span class="font-bold">Created at:</span>
				{new Date(volume.createdAt).toLocaleString()}
			</div>
			<div class="text-ellipsis overflow-hidden flex items-center gap-2 whitespace-nowrap">
				<Fa icon={faFolderOpen} fw />
				<span class="font-bold">Mountpoint:</span>
				{volume.mountpoint}
			</div>
			<div class="text-ellipsis overflow-hidden flex items-center gap-2 whitespace-nowrap">
				<Fa icon={faDatabase} fw />
				<span class="font-bold">Size:</span>
				{volume.size}
			</div>
		</div>
		<div class="flex items-center gap-4">
			<!-- <button class="btn variant-filled-primary" on:click={downloadVolume}>
				<Fa icon={faDownload} fw />
				Download
			</button> -->
			<a href="/volumes/{volume.name}" class="btn variant-ghost p-2">
				<Fa icon={faEllipsisVertical} fw />
			</a>
		</div>
	</div>
</div>
