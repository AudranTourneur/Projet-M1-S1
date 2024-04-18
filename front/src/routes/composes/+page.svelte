<script lang="ts">
	import copy from 'copy-to-clipboard';
	import type { ComposeData } from '$lib/types/ComposeData';
	import type { ComposeList } from '$lib/types/ComposeList';
	import {
		faBox,
		faBoxes,
		faCheck,
		faCopy,
		faCube,
		faEllipsisVertical,
		faPlay
	} from '@fortawesome/free-solid-svg-icons';
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

	let isIdCopied = false;
	let isPathCopied = false;
	const copyToClipboardId = (c) => {
		isIdCopied = true;
		setTimeout(() => (isIdCopied = false), 1000);
		copy(c.id);
	};
	const copyToClipboardPath = (c) => {
		isPathCopied = true;
		setTimeout(() => (isPathCopied = false), 1000);
		copy(c.filePath);
	};
</script>

<h1 class="text-center text-4xl mb-5">Docker Composes</h1>

<div class="mx-auto max-w-xs mb-5">
	<input bind:value={search} type="text" placeholder="Search by Path" class="input" />
</div>

<div>
	{#each visibleComposes as compose}
		<div
			class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4 flex justify-between items-center gap-2">
			<div class="flex flex-col justify-center flex-grow">
				<div class="flex items-center gap-2 w-full">
					<div class="flex items-center justify-center gap-2 w-full">
						<div class="flex flex-col gap-3 w-full">
							<div>
								<div class="copy-to-clipboard">
									<span class="font-bold">Path : </span>
									<a class="text hover:text-gray-500" href="/composes/{compose.id}">{compose.filePath}</a>
									<button
										type="button"
										class="btn variant-soft"
										on:click={() => copyToClipboardPath(compose)}>
										{#if isPathCopied}
											<Fa icon={faCheck} class="text-green-500" />
										{:else}
											<Fa icon={faCopy} />
										{/if}
									</button>
								</div>
								<div class="copy-to-clipboard">
									<span class="font-bold">ID : </span>
									<Tooltip tooltipText={`ID : ${compose.id}`}>
										{compose.id.substring(0, 19)}...
									</Tooltip>
									<button type="button" class="btn variant-soft" on:click={() => copyToClipboardId(compose)}>
										{#if isIdCopied}
											<Fa icon={faCheck} class="text-green-500" />
										{:else}
											<Fa icon={faCopy} />
										{/if}
									</button>
								</div>
							</div>
							<div>
								<span class="font-bold">Number of containers :</span>
								{compose.containers.length} containers
							</div>
							<div class="flex justify-center items-center flex-1 w-full gap-12">
								{#each compose.containers as c}
									<a href="/containers/{c.id}">
										<div
											class="flex flex-col items-center bg-surface-400/40 dark:bg-surface-800/40 border rounded-lg shadow p-4 hover:bg-surface-200/20 dark:hover:bg-surface-600/30">
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
