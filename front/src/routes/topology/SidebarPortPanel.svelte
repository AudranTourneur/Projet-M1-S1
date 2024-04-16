<script lang="ts">
	import { faCheck, faCopy, faCube, faGear } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	import copy from 'copy-to-clipboard';
	import type { TopologyPortPixi } from '$lib/TopologyPortPixi';

	export let entity: TopologyPortPixi;
	const data = entity.data;

</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
	<div>
		<span class="font-bold">ip : </span>
		{data.ip}
	</div>
	<div>
		<span class="font-bold">privatePort : </span>
		{data.privatePort}
	</div>
	<div>
		<span class="font-bold">publicPort : </span>
		{data.publicPort}
	</div>
	<div>
		<span class="font-bold">type : </span>
		{data.type}
	</div>
</div>