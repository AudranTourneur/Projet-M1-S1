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

	const historyTableData: TableSource= {
		head: ['Id', 'Date', "Size", 'Created by'],
		body: tableMapperValues(formatedHistory, ['id', 'created', 'size', 'createdBy'])
	}
</script>

<div class="relative p-3 m-2 shadow rounded-lg overflow-auto">
	<div>
		<span>id: {image.id}</span>
	</div>
	<br />
	<div>
		<span>tags: {image.tags}</span>
	</div>
	<br />
	<div>
		<span>Created : {new Date(image.created).toLocaleString()}</span>
	</div>
	<br />
	<div>
		Size :{formatBytes(image.size)}
	</div>
	<div class="flex justify-end mt-4">
		<input class="input" bind:value={name} placeholder="Container name" />
		<button class="btn variant-ghost-success ml-2" on:click={createNewContainer} disabled={isLoadingCreate}>
			<Fa icon={isLoadingCreate ? faCircleNotch : faPlusCircle} spin={isLoadingCreate} fw class="mr-1" />
			Create a new container from this image
		</button>
	</div>
	<div class="flex justify-end mt-4">
		<button class="btn variant-filled-error ml-6" on:click={deleteVolume} disabled={isLoadingRemove}>
			<Fa icon={isLoadingRemove ? faCircleNotch : faTrash} spin={isLoadingRemove} fw class="mr-1" />
			Delete image
		</button>
	</div>
</div>
<Table source={historyTableData} interactive={true} />
