<script lang="ts">
	import type { TopologyVolumePixi } from '$lib/TopologyVolumePixi';
	import copy from 'copy-to-clipboard';

	import Tooltip from '../../components/Tooltip.svelte';
	import { faCalendarPlus, faCheck, faCopy, faDatabase, faFolderOpen, faFolderPlus, faImage } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let entity: TopologyVolumePixi;

	const data = entity.data.data;

	let isNameCopied = false;
	const copyToClipboardName = () => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(data.name.substring(0, data.name.length));
	};

	let fullName = data.name
	let shortName = data.name.length < 15 ? data.name.substring(0, data.name.length) : (data.name.substring(0, 12) + '...')
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token flex items-center p-3">
	<Fa icon={faDatabase} class="text-5xl" />

	<div class="flex copy-to-clipboard p-4 text-4xl">
		<a href="/volumes/{data.name}" class="btn variant-ghost p-1">
				<Tooltip tooltipText={shortName}>
					<span class="text-xl p-2">{fullName}</span> 
				</Tooltip>
			
		</a>

		<button type="button" class="btn variant-soft" on:click={copyToClipboardName}>
			{#if isNameCopied}
				<Fa icon={faCheck} class="text-green-500" />
			{:else}
				<Fa icon={faCopy} />
			{/if}
		</button>
	</div>
</div>

<div class="flex items-center p-1 gap-3">
	<Fa icon={faFolderPlus} />
	<span class="font-bold">Created at :</span>
	{new Date(data.createdAt).toLocaleString()}
</div>
<div class="flex items-center p-1 gap-3">
	<Fa icon={faFolderOpen} />
	<span class="font-bold">Mountpoint :</span>
	{#if data.mountpoint.length < 20}
		{data.mountpoint.substring(0, data.mountpoint.length)}
	{:else}
		<Tooltip tooltipText={data.mountpoint.substring(0, data.mountpoint.length)}>
			{data.mountpoint.substring(0, 17)}
		</Tooltip>
		<span>...</span>
	{/if}
</div>
<div class="flex items-center p-1 gap-3">
	<Fa icon={faDatabase} />
	<span class="font-bold">Size : </span>
	{data.size}
</div>
