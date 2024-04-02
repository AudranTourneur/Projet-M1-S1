<script lang="ts">
	import type { ComposeData } from '$lib/types/ComposeData';

	export let data: ComposeData;

	let compose = data;
</script>

<div>
	{compose.filePath}
</div>
{#each compose.containers as container}
	<div>
		{container.names.join(', ')}
	</div>
{/each}
