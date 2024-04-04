<script lang="ts">
	import copy from 'copy-to-clipboard';
	import { Fa } from 'svelte-fa';
	import {
		faCheck,
		faCircleNotch,
		faCoins,
		faCopy,
		faCube,
		faEllipsisVertical,
		faGear,
		faImage,
		faLayerGroup,
		faNetworkWired,
		faPlay,
		faPlug,
		faStop
	} from '@fortawesome/free-solid-svg-icons';
	import type { ContainerData } from '$lib/types/ContainerData';
	import Tooltip from '../../components/Tooltip.svelte';

	export let container: ContainerData;
	export let refresh: () => void;

	const dockerComposeName = container.composeFile?.split('/').slice(-2, -1)[0].toLowerCase();

	function concatenatePortConfigurations(configurations: ContainerData['ports']) {
		if (configurations.length === 0) {
			return '-';
		}
		const concatenatedPorts = {};
		configurations.forEach((config) => {
			if (config.publicPort !== null) {
				const key = `${config.publicPort}:${config.privatePort}`;
				// Check if the entry exists already for IPv4 or IPv6
				if (!(key in concatenatedPorts)) {
					concatenatedPorts[key] = true;
				}
			}
		});
		// return the keys of the concatenatedPorts object and sort them to have a consistent order
		return Object.keys(concatenatedPorts).sort().join(', ');
	}

	let isLoadingStart = false;
	let isLoadingStop = false;

	const startContainer = async () => {
		isLoadingStart = true;
		await fetch(`/containers/${container.id}/api/start`, {
			method: 'POST'
		});
		isLoadingStart = false;
		refresh();
	};

	const stopContainer = async () => {
		isLoadingStop = true;
		await fetch(`/containers/${container.id}/api/stop`, {
			method: 'POST'
		});
		isLoadingStop = false;
		refresh();
	};

	let isIdCopied = false;
	let isNameCopied = false;
	const copyToClipboardId = () => {
		isIdCopied = true;
		setTimeout(() => (isIdCopied = false), 1000);
		copy(container.id);
	};
	const copyToClipboardName = () => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(container.names[0].substring(1, container.names[0].length));
	};
</script>

<div
	class="border-token border-surface-300-600-token rounded-container-token p-4 mb-4 flex justify-between items-center gap-2">
	<div class="flex items-center gap-2 md:gap-4">
		<div class="flex flex-col items-center gap-2 w-6">
			{#if container.iconUrl}
				<img src={container.iconUrl} alt={container.image} class="w-6 h-6" />
			{:else}
				<Fa icon={faCube} class="text-surface-300-600-token text-xl" />
			{/if}
			<Tooltip tooltipText={container.status}>
				{#if container.status.includes('Paused')}
					<Fa icon={faGear} class="text-warning-500 animate-pulse text-xl" />
				{:else if container.status.includes('Up')}
					<Fa icon={faGear} class="text-success-500 animate-spin text-xl" style="animation-duration: 5s;" />
				{:else}
					<Fa icon={faGear} class="text-error-400 text-xl" />
				{/if}
			</Tooltip>
		</div>
		<div class="flex flex-col w-36">
			<div class="font-bold copy-to-clipboard">
				{#if container.names[0].length < 15}
					{container.names[0].substring(1, container.names[0].length)}
				{:else}
					<Tooltip tooltipText={container.names[0].substring(1, container.names[0].length)}>
						{container.names[0].substring(1, 12)}
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
			<div class="copy-to-clipboard">
				<Tooltip tooltipText={`Container ID: ${container.id}`}>
					{container.id.substring(0, 12)}
				</Tooltip>
				<div class="hide-on-clipboard-hover">...</div>
				<button type="button" class="btn variant-soft" on:click={copyToClipboardId}>
					{#if isIdCopied}
						<Fa icon={faCheck} class="text-green-500" />
					{:else}
						<Fa icon={faCopy} />
					{/if}
				</button>
			</div>
		</div>
	</div>
	<div class="flex flex-wrap gap-2 flex-grow min-w-0">
		{#if dockerComposeName}
			<Tooltip
				class="chip variant-soft-primary overflow-hidden px-1.5 lg:px-2.5"
				tooltipText={`Docker Compose: ${container.composeFile}`}>
				<Fa icon={faLayerGroup} />
				<span class="text-ellipsis overflow-hidden">{dockerComposeName}</span>
			</Tooltip>
		{/if}
		<Tooltip
			class="chip variant-soft overflow-hidden px-1.5 lg:px-2.5"
			tooltipText={`Image: ${container.image}`}>
			<Fa icon={faImage} />
			<span class="text-ellipsis overflow-hidden">{container.image}</span>
		</Tooltip>
		<Tooltip
			class="chip variant-soft overflow-hidden px-1.5 lg:px-2.5"
			tooltipText={`Networks: ${container.networks.length === 0 ? ' - ' : container.networks.join(', ')}`}>
			<Fa icon={faNetworkWired} />
			<span class="text-ellipsis overflow-hidden">
				{container.networks.length === 0 ? ' - ' : container.networks.join(', ')}
			</span>
		</Tooltip>
		<Tooltip
			class="chip variant-soft overflow-hidden px-1.5 lg:px-2.5"
			tooltipText={`Ports: ${concatenatePortConfigurations(container.ports)}`}>
			<Fa icon={faPlug} />
			<span class="text-ellipsis overflow-hidden">
				{concatenatePortConfigurations(container.ports)}
			</span>
		</Tooltip>
		<Tooltip
			class="chip variant-soft overflow-hidden px-1.5 lg:px-2.5"
			tooltipText={`Volumes: ${container.volumes.length === 0 ? ' - ' : container.volumes.join(', ')}`}>
			<Fa icon={faCoins} />
			<span class="text-ellipsis overflow-hidden">
				{container.volumes.length === 0 ? ' - ' : container.volumes.join(', ')}
			</span>
		</Tooltip>
	</div>
	<div class="flex gap-1">
		<button
			class="btn variant-ghost-success p-2"
			disabled={container.status.includes('Up') || isLoadingStart}
			on:click={startContainer}>
			<Fa icon={!isLoadingStart ? faPlay : faCircleNotch} class={isLoadingStart ? 'animate-spin' : ''} fw />
		</button>
		<button
			class="btn variant-ghost-error p-2"
			disabled={container.status.includes('Exited') || isLoadingStop}
			on:click={stopContainer}>
			<Fa icon={!isLoadingStop ? faStop : faCircleNotch} class={isLoadingStop ? 'animate-spin' : ''} fw />
		</button>
		<a href="/containers/{container.id}" class="btn variant-ghost p-2">
			<Fa icon={faEllipsisVertical} fw />
		</a>
	</div>
</div>
