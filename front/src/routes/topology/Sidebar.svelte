<script lang="ts">
	import { TopologyContainerPixi } from '$lib/TopologyContainerPixi';
	import { TopologyNetworkPixi } from '$lib/TopologyNetworkPixi';
	import { currentlySelectedEntity } from '$lib/TopologyStore';
	import { TopologyVolumePixi } from '$lib/TopologyVolumePixi';
	import Tooltip from '../../components/Tooltip.svelte';
	import type { TopologyApp } from '$lib/TopologyApp';
	import SidebarContainerPanel from './SidebarContainerPanel.svelte';
	import SidebarVolumePanel from './SidebarVolumePanel.svelte';
	import { TopologyPortPixi } from '$lib/TopologyPortPixi';
	import SidebarNetworkPanel from './SidebarNetworkPanel.svelte';
	import SidebarPortPanel from './SidebarPortPanel.svelte';

	export let app: TopologyApp;

	function unselect() {
		const entity = $currentlySelectedEntity?.entity;
		if (entity) {
			entity.unselect();
		}
		$currentlySelectedEntity = null;
	}

	async function save() {
		const res = await fetch('/topology/api/save', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(app.getSaveData())
		});
	}
</script>

<div
	class="w-[400px] h-full flex flex-col justify-between border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3">
	<div>
		{#key $currentlySelectedEntity}
			{#if $currentlySelectedEntity?.entity instanceof TopologyContainerPixi}
				<SidebarContainerPanel entity={$currentlySelectedEntity?.entity}/>
			{:else if $currentlySelectedEntity?.entity instanceof TopologyNetworkPixi}
				<SidebarNetworkPanel entity={$currentlySelectedEntity?.entity} />
			{:else if $currentlySelectedEntity?.entity instanceof TopologyVolumePixi}
				<SidebarVolumePanel entity={$currentlySelectedEntity?.entity} />
			{:else if $currentlySelectedEntity?.entity instanceof TopologyPortPixi}
				<SidebarPortPanel entity={$currentlySelectedEntity?.entity} />
			{:else}
				Please select something to get more details.
			{/if}
		{/key}
	</div>

	<div class="flex justify-center p-3">
		{#if $currentlySelectedEntity === null}
			<button type="button" class="btn justify-end variant-filled-primary" on:click={save}>Save</button>
		{:else}
			<button type="button" class="btn justify-end variant-filled" on:click={unselect}>Unselect</button>
		{/if}
	</div>
</div>
