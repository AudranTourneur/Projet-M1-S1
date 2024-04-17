<script lang="ts">
	import { faCheck, faCopy, faCube, faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import copy from 'copy-to-clipboard';
	import type { TopologyPortPixi } from '$lib/TopologyPortPixi';
	import { TopologyContainerPixi } from '$lib/TopologyContainerPixi';

	export let entity: TopologyPortPixi;
	const data = entity.data;

	const linkContainer = entity.links[0].source;

	let isNameCopied = false;
	let isIdCopied = false;
	const copyToClipboardName = (linkContainer: TopologyContainerPixi) => {
		isNameCopied = true;
		setTimeout(() => (isNameCopied = false), 1000);
		copy(linkContainer.data.data.names[0].substring(1, linkContainer.data.data.names[0].length));
	};
	const copyToClipboardId = (linkContainer: TopologyContainerPixi) => {
		isIdCopied = true;
		setTimeout(() => (isIdCopied = false), 1000);
		copy(linkContainer.data.data.id);
	};
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 flex items-center mb-4">
	<!-- svelte-ignore a11y-missing-attribute -->
	<img src="/static/plug.svg" width="50" height="40" />
	<div class="flex-col p-4">
		<div>
			<span class="font-bold">IP : </span>
			{data.ip}
		</div>
		<div>
			<span class="font-bold">Private Port : </span>
			{data.privatePort}
		</div>
		<div>
			<span class="font-bold">Public Port : </span>
			{data.publicPort}
		</div>
		<div>
			<span class="font-bold">Type : </span>
			{data.type}
		</div>
	</div>
</div>

{#if linkContainer instanceof TopologyContainerPixi}
	<div class="flex justify-center font-bold">
		<span>Container Link to this Port :</span>
	</div>

	<div class="flex flex-col">
		<div class="copy-to-clipboard">
			<span class="font-bold p-2">Name :</span>

			{#if linkContainer.data.data.names[0].length < 25}
				{linkContainer.data.data.names[0].substring(0, linkContainer.data.data.names[0].length)}
			{:else}
				<Tooltip
					tooltipText={linkContainer.data.data.names[0].substring(
						1,
						linkContainer.data.data.names[0].length
					)}>
					{linkContainer.data.data.names[0].substring(0, 22)}
				</Tooltip>
				<div class="hide-on-clipboard-hover">...</div>
			{/if}
			<button type="button" class="btn variant-soft" on:click={() => copyToClipboardName(linkContainer)}>
				{#if isNameCopied}
					<Fa icon={faCheck} class="text-green-500" />
				{:else}
					<Fa icon={faCopy} />
				{/if}
			</button>
		</div>
	</div>
	<div class="copy-to-clipboard">
		<span class="font-bold p-2">ID :</span>

		<a href="/containers/{linkContainer.data.data.id}" class="btn variant-ghost p-1">
			<Tooltip tooltipText={`Container ID: ${linkContainer.data.data.id}`}>
				{linkContainer.data.data.id.substring(0, 12)}
			</Tooltip>
			<span>...</span>
		</a>

		<button type="button" class="btn variant-soft" on:click={() => copyToClipboardId(linkContainer)}>
			{#if isNameCopied}
				<Fa icon={faCheck} class="text-green-500" />
			{:else}
				<Fa icon={faCopy} />
			{/if}
		</button>
	</div>
{/if}
