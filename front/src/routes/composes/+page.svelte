<script lang="ts">
	import type { ComposeData } from '$lib/types/ComposeData';
	import type { ComposeList } from '$lib/types/ComposeList';
	import { faBox, faBoxes, faCube, faEllipsisVertical, faPlay } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import Tooltip from '../../components/Tooltip.svelte';
	export let data: ComposeList;

	let composes = data.composes;
	let search = '';
	let visibleComposes = [...composes];

	console.log(composes);

	async function startCompose(c: ComposeData) {
		await fetch(`/composes/${c.id}/api/start`, { method: 'POST' });
	}

	$: visibleComposes = composes.filter((compose) => {
		const searchString = search.toLowerCase();
		return compose.filePath.toLowerCase().includes(searchString);
	});
</script>

<h1 class="text-center text-4xl mb-5">Docker Composes</h1>

<div class="mx-auto max-w-xs mb-5">
	<input bind:value={search} type="text" placeholder="Search by Path" class="input" />
</div>

<div>
	{#each visibleComposes as compose}
		<div
			class="flex justify-between items-center gap-2 p-2 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border">
			<div class="flex flex-col justify-center flex-grow">
				<div class="flex items-center gap-2 w-full">
					<div class="flex items-center justify-center gap-2 w-full">
						<div class="flex flex-col gap-3 w-full" >
							<div>
								<span class="font-bold">Path : </span>
								<a class="text hover:text-gray-500" href="/composes/{compose.id}">{compose.filePath}</a>
								<br />
								<span class="font-bold">ID : </span>
								<Tooltip tooltipText={`ID : ${compose.id}`}>
									{compose.id.substring(0, 19)}...
								</Tooltip>
							</div>
							<div>
								<span class="font-bold">Number of containers :</span>
								{compose.containers.length} containers
							</div>
							<div
								class="flex justify-center items-center flex-1 w-full gap-12"
								>
								{#each compose.containers as c}
									<a href="/containers/{c.id}">
										<div class="flex flex-col items-center bg-surface-400/40 dark:bg-surface-800/40 border rounded-lg shadow p-4 hover:bg-surface-200/20 dark:hover:bg-surface-600/30">
											<Fa size="2x" icon={faCube}></Fa>
											<span class="text-md hover:text-gray-500">{c.names[0]}</span>
										</div>
									</a>
								{/each}
							</div>
						</div>
					</div>
					<div class="ml-auto">
						<a href="/composes/{compose.id}" class="btn variant-ghost p-2 mb-2">
							<Fa icon={faEllipsisVertical} fw />
						</a>
						<!-- Button to start a compose, don't know if the function is added yet or not-->
						<!-- <button class="btn variant-filled-success p-2" on:click={() => startCompose(compose)}>
							<Fa icon={faPlay} fw />
						</button> -->
					</div>
				</div>
			</div>
		</div>
	{/each}
</div>
