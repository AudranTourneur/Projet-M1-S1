<script lang="ts">
	import {
		faCheck,
		faCircleNotch,
		faCopy,
		faCube,
		faDatabase,
		faGear,
		faImage,
		faNetworkWired,
		faPlay,
		faStop
	} from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import type { TopologyContainerPixi } from '$lib/TopologyContainerPixi';
	import copy from 'copy-to-clipboard';
	import ContainerStatusIcon from '../containers/ContainerStatusIcon.svelte';
	import { getContainerActionsFromStatus } from '../containers/getContainerActionsFromStatus';
	import ContainerChart from '../../components/Chart.svelte';
	import Chart from '../../components/Chart.svelte';

	export let entity: TopologyContainerPixi;

	let data = entity.data.data;

	const { statusIcon, canBeStarted, canBeStopped } = getContainerActionsFromStatus(data.status);

	let isNameCopied = false;
	let isIdCopied = false;
	const copyToClipboardName = () => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(data.names[0].substring(1, data.names[0].length));
	};
	const copyToClipboardId = () => {
		isIdCopied = true;
		setTimeout(() => (isIdCopied = false), 1000);
		copy(data.id);
	};

	let heightDivVolume = 'max-h';
	if (data.volumes.length > 3) {
		heightDivVolume = 'h-[75px]';
	}

	let isLoadingStart = false;
	let isLoadingStop = false;

	const startContainer = async () => {
		isLoadingStart = true;
		await fetch(`/containers/${data.id}/api/start`, {
			method: 'POST'
		});
		isLoadingStart = false;
	};

	const stopContainer = async () => {
		isLoadingStop = true;
		await fetch(`/containers/${data.id}/api/stop`, {
			method: 'POST'
		});
		isLoadingStop = false;
	};
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token flex flex-col items-center p-3 gap-2">
	{#if data.iconUrl}
		<img src={data.iconUrl} alt={data.image} class="max-w-full max-h-[60px]" />
	{:else}
		<Fa icon={faCube} class="text-surface-400-500-token text-xl" />
	{/if}

	<div class="flex flex-col">
		<div class="copy-to-clipboard">
			{#if data.names[0].length < 25}
				{data.names[0].substring(0, data.names[0].length)}
			{:else}
				<Tooltip tooltipText={data.names[0].substring(1, data.names[0].length)}>
					{data.names[0].substring(0, 22)}
				</Tooltip>
				<div class="hide-on-clipboard-hover">...</div>
			{/if}
			<button type="button" class="btn variant-soft" on:click={copyToClipboardName}>
				{#if isNameCopied}
					<Fa icon={faCheck} class="text-green-500" />
				{:else}
					<Fa icon={faCopy} />
				{/if}
			</button>
		</div>
	</div>
	<div class="copy-to-clipboard">
		<a href="/containers/{data.id}" class="btn variant-ghost p-1">
			<Tooltip tooltipText={`Container ID: ${data.id}`}>
				{data.id.substring(0, 12)}
			</Tooltip>
			<span>...</span>
		</a>

		<button type="button" class="btn variant-soft" on:click={copyToClipboardId}>
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
	<span class="font-bold">Image ID : </span>
	{#if data.image.length < 25}
		{data.image.substring(0, data.image.length)}
	{:else}
		<Tooltip tooltipText={data.image.substring(0, data.image.length)}>
			{data.image.substring(0, 22)}
		</Tooltip>
		<span>...</span>
	{/if}
</div>
<div class="flex items-center p-1 gap-3">
	<ContainerStatusIcon status={statusIcon} statusString={data.status} />
	<span class="font-bold">Status :</span>
	{data.status}
</div>
<div class="flex items-center p-1 gap-3">
	<Fa icon={faNetworkWired} />
	<span class="font-bold">Networks : </span>
	{data.networks.join(', ')}
</div>

<!-- svelte-ignore a11y-missing-attribute -->
<div class="flex items-center p-1 gap-3">
	<Fa icon={faDatabase} />
	<span class="font-bold">Volumes : </span>

	<div
		class="overflow-y-auto {heightDivVolume} border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token">
		{#each data.volumes as volume}
			<div class="scroll-auto">
				{#if volume.length < 25}
					{volume}
				{:else}
					<Tooltip tooltipText={volume.substring(0, volume.length)}>
						{volume.substring(0, 22)}
					</Tooltip>
					<span>...</span>
				{/if}
				<br />
			</div>
		{/each}
	</div>
</div>
<div class="flex overflow-y-auto h-[400px]">
	<Chart containerID={data.id} typeChart="Mem" />
	<Chart containerID={data.id} typeChart="Cpu" />
	<Chart containerID={data.id} typeChart="Io" />
	<Chart containerID={data.id} typeChart="Net" />
</div>

<button
	class="btn variant-filled-success p-2"
	disabled={!canBeStarted || isLoadingStart}
	on:click={startContainer}>
	<Fa icon={!isLoadingStart ? faPlay : faCircleNotch} class={isLoadingStart ? 'animate-spin' : ''} fw />
</button>
<button
	class="btn variant-filled-error p-2"
	disabled={!canBeStopped || isLoadingStop}
	on:click={stopContainer}>
	<Fa icon={!isLoadingStop ? faStop : faCircleNotch} class={isLoadingStop ? 'animate-spin' : ''} fw />
</button>
