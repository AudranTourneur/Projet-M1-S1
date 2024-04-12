<script lang="ts">
	import { faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import type { TopologyContainerPixi } from '$lib/TopologyContainerPixi';

	export let entity: TopologyContainerPixi;
	const data = entity.data.data;
</script>

<a href="/containers/{data.id}" class="btn variant-ghost p-2">
	<Tooltip tooltipText={`Container ID: ${data.id}`}>
		{data.id.substring(0, 12)}
	</Tooltip>
</a>
<Tooltip tooltipText={data.status}>
	{#if data.status.includes('Paused')}
		<Fa icon={faGear} class="text-warning-500 animate-pulse text-xl" />
	{:else if data.status.includes('Up')}
		<Fa icon={faGear} class="text-success-500 animate-spin text-xl" style="animation-duration: 5s;" />
	{:else}
		<Fa icon={faGear} class="text-error-400 text-xl" />
	{/if}
</Tooltip>

{#if data.names[0].length < 15}
	{data.names[0].substring(1, data.names[0].length)}
{:else}
	<Tooltip tooltipText={data.names[0].substring(1, data.names[0].length)}>
		{data.names[0].substring(1, 12)}
	</Tooltip>
	<div class="hide-on-clipboard-hover">...</div>
{/if}
