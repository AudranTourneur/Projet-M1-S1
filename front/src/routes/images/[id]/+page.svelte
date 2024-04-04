<script lang="ts">
	import { formatBytes, formatCreatedDate } from '$lib/FormatUtils';

	export let data;

	console.log(data);

	let image = data;

	import { onMount } from 'svelte';

	onMount(() => {
		console.log('todo');
	});

	let name = '';
	async function CreateNewContainer() {
		const res = await fetch('/images/' + data.id + '/api/create-container', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				containerName: !!name ? name : null
			})
		});
		console.log(res);
	}


	function deleteVolume(i: any): any {
		throw new Error('Function not implemented.');
	}
</script>

<div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg overflow-auto">
	<div>
		<span>id: {image.id}</span>
	</div>
	<br />
	<div>
		<span>tags: {image.tags}</span>
	</div>
	<br />
	<div>
		<span>Created :{formatCreatedDate(image.created)}</span>
	</div>
	<br />
	<div>
		Size :{formatBytes(image.size)}
	</div>
	<div class="flex justify-end">
		<input bind:value={name} placeholder="enter your name" />
		<button class="btn variant-ghost-success p-2" on:click={CreateNewContainer}>
			Create a New Container
		</button>

		<a href="/images/{image.id}">
			<button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info </button>
		</a>
		<button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => deleteVolume(i)}
			>Delete</button>
	</div>
</div>
