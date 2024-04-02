<script lang="ts">
	import type { ComposeList } from '$lib/types/ComposeList';
	import { faBox, faBoxes, faCube } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let data: ComposeList;

	let composes = data.composes;
</script>

<div class="flex flex-col gap-24">
	{#each composes as compose}
		<div class="flex flex-col gap-3">
			<a class="text-xl" href="/composes/{compose.id}">{compose.filePath}</a>
			{compose.containers.length} containers
			<div class="grid grid-cols-7 gap-12 w-[1400px]">
				{#each compose.containers as c}
					<div class="flex flex-col items-center">
						<Fa size="2x" icon={faCube}></Fa>
						<span class="text-md">{c.names[0]}</span>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>
