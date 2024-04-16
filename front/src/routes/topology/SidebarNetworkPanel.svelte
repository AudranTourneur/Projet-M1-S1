<script lang="ts">
	import { faCheck, faCopy, faCube, faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import copy from 'copy-to-clipboard';
	import type { TopologyNetworkPixi } from '$lib/TopologyNetworkPixi';

	export let entity: TopologyNetworkPixi;
	const data = entity;

</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3">
	<div>
		<span class="font-bold">Name : </span>
		{data.name}
	</div>
</div>