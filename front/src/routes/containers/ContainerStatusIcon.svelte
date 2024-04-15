<script lang="ts">
	import { faGear } from '@fortawesome/free-solid-svg-icons';
	import Tooltip from '../../components/Tooltip.svelte';
	import { Fa } from 'svelte-fa';

	export let status: 'running' | 'paused' | 'stopped';
	export let statusString: string;
</script>

<Tooltip tooltipText={statusString}>
	{#if status === 'paused'}
		<Fa icon={faGear} class="text-warning-500 animate-pulse text-xl" />
	{:else if status === 'running'}
		<Fa icon={faGear} class="text-success-500 animate-spin text-xl" style="animation-duration: 5s;" />
	{:else}
		<Fa icon={faGear} class="text-error-400 text-xl" />
	{/if}
</Tooltip>
