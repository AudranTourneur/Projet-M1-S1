<script lang="ts">
	import type { TopologyApp } from '$lib/TopologyApp.js';
	import { TopologyContainerPixi } from '$lib/TopologyContainerPixi.js';
	import { TopologyNetworkPixi } from '$lib/TopologyNetworkPixi.js';
	import { currentlySelectedEntity } from '$lib/TopologyStore.js';
	import { TopologyVolumePixi } from '$lib/TopologyVolumePixi.js';
	import { onMount } from 'svelte';
	import Tooltip from '../../components/Tooltip.svelte';
	import { faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { Container } from 'pixi.js';
	import { TopologyEntityPixi } from '$lib/TopologyEntityPixi';

	let parentElement: HTMLDivElement;
	let canvasElement: HTMLCanvasElement;

	export let data;

	let app: TopologyApp;

	onMount(async () => {
		const { initApp } = await import('$lib/topology');
		app = initApp(canvasElement, parentElement, data);

		console.log('Topology data', data);
	});

	async function save() {
		console.log('save');
		const res = await fetch('/topology/api/save', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(app.getSaveData())
		});
		console.log('RES', await res.text());
	}

	async function unselect() {
		console.log($currentlySelectedEntity);
		currentlySelectedEntity.set(null);
	}
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4 flex justify-between items-center gap-2">
	<div bind:this={parentElement} class="basis-2/3 justify-end w-[80vw] h-[72vh]">
		<canvas bind:this={canvasElement} id="topology-canvas" />
	</div>

	<div class="basis-1/3 items-center auto-cols-auto">
		{#if $currentlySelectedEntity?.entity instanceof TopologyContainerPixi}
			<a href="/containers/{$currentlySelectedEntity?.entity.data.data.id}" class="btn variant-ghost p-2">
				<Tooltip tooltipText={`Container ID: ${$currentlySelectedEntity?.entity.data.data.id}`}>
					{$currentlySelectedEntity?.entity.data.data.id.substring(0, 12)}
				</Tooltip>
			</a>
			<Tooltip tooltipText={$currentlySelectedEntity?.entity.data.data.status}>
				{#if $currentlySelectedEntity?.entity.data.data.status.includes('Paused')}
					<Fa icon={faGear} class="text-warning-500 animate-pulse text-xl" />
				{:else if $currentlySelectedEntity?.entity.data.data.status.includes('Up')}
					<Fa icon={faGear} class="text-success-500 animate-spin text-xl" style="animation-duration: 5s;" />
				{:else}
					<Fa icon={faGear} class="text-error-400 text-xl" />
				{/if}
			</Tooltip>

			{#if $currentlySelectedEntity?.entity.data.data.names[0].length < 15}
				{$currentlySelectedEntity?.entity.data.data.names[0].substring(
					1,
					$currentlySelectedEntity?.entity.data.data.names[0].length
				)}
			{:else}
				<Tooltip
					tooltipText={$currentlySelectedEntity?.entity.data.data.names[0].substring(
						1,
						$currentlySelectedEntity?.entity.data.data.names[0].length
					)}>
					{$currentlySelectedEntity?.entity.data.data.names[0].substring(1, 12)}
				</Tooltip>
				<div class="hide-on-clipboard-hover">...</div>
			{/if}
			<button type="button" class="btn justify-end variant-filled" on:click={unselect}>déselctionner</button>
		{/if}

		{#if $currentlySelectedEntity?.entity instanceof TopologyNetworkPixi}
			{$currentlySelectedEntity?.entity.name}
			<button type="button" class="btn justify-end variant-filled" on:click={unselect}>déselctionner</button>
		{/if}

		{#if $currentlySelectedEntity?.entity instanceof TopologyVolumePixi}
			{#if $currentlySelectedEntity?.entity.data.data.name.length < 15}
				{$currentlySelectedEntity?.entity.data.data.name.substring(
					1,
					$currentlySelectedEntity?.entity.data.data.name.length
				)}
			{:else}
				<Tooltip
					tooltipText={$currentlySelectedEntity?.entity.data.data.name.substring(
						1,
						$currentlySelectedEntity?.entity.data.data.name.length
					)}>
					{$currentlySelectedEntity?.entity.data.data.name.substring(1, 12)}
				</Tooltip>
				<div class="hide-on-clipboard-hover">...</div>
				<button type="button" class="btn justify-end variant-filled" on:click={unselect}
					>déselctionner</button>
			{/if}
		{/if}

		{#if $currentlySelectedEntity === null}
			<button type="button" class="btn justify-end variant-filled" on:click={save}>Save</button>
		{/if}
	</div>
</div>
