<script lang="ts">
	import copy from 'copy-to-clipboard';
	import { Fa } from 'svelte-fa';
	import {
		faCheck,
		faCoins,
		faCopy,
		faEllipsisVertical,
		faGear,
		faNetworkWired,
		faPlay,
		faPlug,
		faStop
	} from '@fortawesome/free-solid-svg-icons';
	import { PUBLIC_API_URL } from '$env/static/public';
	import type { ContainerData } from '$lib/types/ContainerData';
	import Tooltip from '../../components/Tooltip.svelte';

	export let container: ContainerData;
	export let refresh: () => void;

	// console.log('Container', container);

	const startContainer = async () => {
		await fetch(`${PUBLIC_API_URL}/container/${container.id}/start`, {
			method: 'POST',
			mode: 'no-cors',
			headers: {
				'Content-Type': 'application/json'
			}
		});
		refresh();
	};

	const stopContainer = async () => {
		await fetch(`${PUBLIC_API_URL}/container/${container.id}/stop`, {
			method: 'POST',
			mode: 'no-cors',
			headers: {
				'Content-Type': 'application/json'
			}
		});
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
		<Tooltip tooltipText={container.status}>
			{#if container.isRunning}
				<Fa icon={faGear} class="text-success-500 animate-spin text-2xl" style="animation-duration: 4s;" />
			{:else}
				<Fa icon={faGear} class="text-error-400 text-2xl" />
			{/if}
		</Tooltip>
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
				<Tooltip tooltipText={container.id}>
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
			tooltipText={`Ports: ${
				container.ports.length === 0 ? ' - ' : container.ports.map((port) => port.publicPort).join(', ')
			}`}>
			<Fa icon={faPlug} />
			<span class="text-ellipsis overflow-hidden">
				{container.ports.length === 0 ? ' - ' : container.ports.map((port) => port.publicPort).join(', ')}
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
		<button class="btn variant-ghost-success p-2" disabled={container.isRunning} on:click={startContainer}>
			<Fa icon={faPlay} fw />
		</button>
		<button class="btn variant-ghost-error p-2" disabled={!container.isRunning} on:click={stopContainer}>
			<Fa icon={faStop} fw />
		</button>
		<a href="/containers/{container.id}" class="btn variant-ghost p-2">
			<Fa icon={faEllipsisVertical} fw />
		</a>
	</div>
</div>
