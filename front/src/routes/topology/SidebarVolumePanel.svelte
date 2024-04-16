<script lang="ts">
	import type { TopologyVolumePixi } from '$lib/TopologyVolumePixi';
	import copy from 'copy-to-clipboard';

	import Tooltip from '../../components/Tooltip.svelte';
	import { faCheck, faCopy, faDatabase, faImage } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let entity: TopologyVolumePixi;

	const data = entity.data.data;

	let isNameCopied = false;
	const copyToClipboardName = () => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(data.name.substring(0, data.name.length));
	};
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token flex items-center p-3 h-[45px]">
	<div class="w-[35px]">
		<Fa icon={faDatabase} class="text-3xl" />
	</div>

	<div class="flex copy-to-clipboard">
		<a href="/volumes/{data.name}" class="btn variant-ghost p-1">
			{#if data.name.length < 15}
				{data.name.substring(0, data.name.length)}
			{:else}
				<Tooltip tooltipText={data.name.substring(1, data.name.length)}>
					{data.name.substring(0, 12)}
				</Tooltip>
				<span>...</span>
			{/if}
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
	<Fa icon={faImage} />
	<span class="font-bold">Created at :</span>
	{new Date(data.createdAt).toLocaleString()}
</div>
<div class="flex items-center p-1 gap-3">
	<Fa icon={faImage} />
	<span class="font-bold">Mountpoint :</span>
	{#if data.mountpoint.length < 15}
		{data.mountpoint.substring(0, data.mountpoint.length)}
	{:else}
		<Tooltip tooltipText={data.mountpoint.substring(0, data.mountpoint.length)}>
			{data.mountpoint.substring(0, 12)}
		</Tooltip>
		<span>...</span>
	{/if}
</div>
<div class="flex items-center p-1 gap-3">
	<Fa icon={faImage} />
	<span class="font-bold">Size : </span>
	{data.size}
</div>
