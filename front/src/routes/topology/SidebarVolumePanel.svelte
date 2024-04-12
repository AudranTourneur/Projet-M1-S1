<script lang="ts">
	import type { TopologyVolumePixi } from '$lib/TopologyVolumePixi';

	import Tooltip from '../../components/Tooltip.svelte';

	export let entity: TopologyVolumePixi;

	const data = entity.data.data;
</script>

{#if data.name.length < 15}
	{data.name.substring(1, data.name.length)}
{:else}
	<Tooltip tooltipText={data.name.substring(1, data.name.length)}>
		{data.name.substring(1, 12)}
	</Tooltip>
	<div class="hide-on-clipboard-hover">...</div>
{/if}
