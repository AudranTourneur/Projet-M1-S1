<script lang="ts">
	import { TopologyContainerPixi } from '$lib/TopologyContainerPixi';
	import { TopologyNetworkPixi } from '$lib/TopologyNetworkPixi';
	import { currentlySelectedEntity } from '$lib/TopologyStore';
	import { TopologyVolumePixi } from '$lib/TopologyVolumePixi';
	import Tooltip from '../../components/Tooltip.svelte';
	import type { TopologyApp } from '$lib/TopologyApp';
	import SidebarContainerPanel from './SidebarContainerPanel.svelte';
	import SidebarVolumePanel from './SidebarVolumePanel.svelte';

	export let app: TopologyApp;

	function unselect() {
		console.log($currentlySelectedEntity);
		$currentlySelectedEntity = null;
	}

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
</script>

<div class="w-[400px] h-full flex flex-col justify-between border border-blue-500">
	<div>
		{#key $currentlySelectedEntity}
			{#if $currentlySelectedEntity?.entity instanceof TopologyContainerPixi}
				<SidebarContainerPanel entity={$currentlySelectedEntity?.entity} />
			{:else if $currentlySelectedEntity?.entity instanceof TopologyNetworkPixi}
				{$currentlySelectedEntity?.entity.name}
				TODO
			{:else if $currentlySelectedEntity?.entity instanceof TopologyVolumePixi}
				<SidebarVolumePanel entity={$currentlySelectedEntity?.entity} />
			{:else}
				Please select something to get more details.
			{/if}
		{/key}
	</div>

	<div class="border border-red-500 flex justify-center">
		{#if $currentlySelectedEntity === null}
			<button type="button" class="btn justify-end variant-filled-primary" on:click={save}>Save</button>
		{:else}
			<button type="button" class="btn justify-end variant-filled" on:click={unselect}>Unselect</button>
		{/if}
	</div>
</div>
