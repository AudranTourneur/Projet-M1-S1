<script lang="ts">
	import { faCheck, faCopy, faCube, faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import copy from 'copy-to-clipboard';
	import type { TopologyNetworkPixi } from '$lib/TopologyNetworkPixi';
	import type { TopologyContainerPixi } from '$lib/TopologyContainerPixi';

	export let entity: TopologyNetworkPixi;
	const data = entity;

	function getAllLinkedContainers(): TopologyContainerPixi[] {
		return entity.app.allContainers.filter((container) => container.data.data.networks.includes(data.name));
	}

	let containers = getAllLinkedContainers();
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token flex items-center p-3">
	<!-- svelte-ignore a11y-missing-attribute -->
	<img src="/static/router.svg" width="50" height="40" />

	<div class="p-2">
		{data.name}
	</div>

	<br />
</div>

<div class="ml-2 flex flex-col gap-3 mt-8">
	{#each containers as container}
		<div>
			{container.data.data.names} <br />
		</div>
	{/each}
</div>
