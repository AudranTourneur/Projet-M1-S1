<script lang="ts">
	import { formatBytes } from '$lib/FormatUtils';
	import Fa from 'svelte-fa';
	import { faCircleNotch, faPlusCircle, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { goto } from '$app/navigation';
	import { Table, type TableSource, tableMapperValues } from '@skeletonlabs/skeleton';

	export let data;

	console.log(data);

	let image = data;

	let name = '';
	let isLoadingCreate = false;
	let isLoadingRemove = false;

	const createNewContainer = async () => {
		isLoadingCreate = true;
		const res = await fetch('/images/' + data.id + '/api/create-container', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				containerName: !!name ? name : null
			})
		});
		goto('/containers/' + (await res.json()));
		isLoadingCreate = false;
	};

	const deleteVolume = async () => {
		isLoadingRemove = true;
		await fetch(`/images/${image.id}/api/remove`, {
			method: 'POST'
		});
		isLoadingRemove = false;
		goto('/images');
	};

	const formatedHistory = image.history?.map((h) => {
		return {
			id: h.id === '<missing>' ? 'missing' : h.id.substring(0, 20) + '...',
			createdBy: h.createdBy,
			created: new Date(h.created).toLocaleString(),
			size: formatBytes(h.size)
		};
	});

	const historyTableData: TableSource = {
		head: ['Id', 'Date', 'Size', 'Created by'],
		body: tableMapperValues(formatedHistory, ['id', 'created', 'size', 'createdBy'])
	};
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4 flex justify-start items-center gap-5">
	{#if image.iconUrl}
		<img src={image.iconUrl} alt="icon" class="max-w-[100px] max-h-[100px]" />
	{/if}
	<div class="overflow-hidden">
		<div class="flex flex-wrap gap-2 items-center mb-3">
			<h1 class="font-heading-token text-lg md:text-2xl chip variant-filled-primary">{image.tags}</h1>
			<button class="btn variant-filled-error py-1 px-3" on:click={deleteVolume} disabled={isLoadingRemove}>
				<Fa icon={isLoadingRemove ? faCircleNotch : faTrash} spin={isLoadingRemove} fw class="mr-1" />
				Delete image
			</button>
		</div>
		<div class="italic mb-3 text-ellipsis overflow-hidden">{image.id}</div>
		<div>Created : {new Date(image.created).toLocaleString()}</div>
		<div>Size : {formatBytes(image.size)}</div>
	</div>
</div>
<div class="flex flex-col sm:flex-row gap-3 my-8">
	<input class="input" bind:value={name} placeholder="Container name" />
	<button class="btn variant-ghost-success" on:click={createNewContainer} disabled={isLoadingCreate}>
		<Fa icon={isLoadingCreate ? faCircleNotch : faPlusCircle} spin={isLoadingCreate} fw class="mr-1" />
		Create a new container from this image
	</button>
</div>
<Table source={historyTableData} interactive={true} />
