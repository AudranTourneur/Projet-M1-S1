<script lang="ts">
	import { onMount } from 'svelte';
	import { Base64 } from 'js-base64';
	import Fa from 'svelte-fa';
	import { faFolder, faFolderOpen, faFile, faArrowLeft } from '@fortawesome/free-solid-svg-icons';

	export let id: string;
	let current_directory = false;
	let path = '/';
	let currentFolder = '/';
	let files: any[] = [];
	let directories: any[] = [];
	let explorer_dir: any[] = [];
	let explorer_file: any[] = [];
	let res;

	onMount(async () => {
		let response = await fetch(`/volumes/${id}/filesystem/${Base64.encodeURI(path)}/api`);
		res = await response.json();
		update(res);
	});

	async function changePage(path: string) {
		if (path == Base64.encodeURI('/' + id)) {
			path = '/';
		}

		const urlApi = `/volumes/${id}/filesystem/${path}/api`;
		console.log('making call to', urlApi);
		let response = await fetch(urlApi);
		res = await response.json();
		update(res);
	}

	function update(res) {
		files = res.files;
		directories = res.directories;
		currentFolder = res.currentFolder;
		console.log(res);
		explorer_dir = [];
		explorer_file = [];

		// Push directories to explorer array with their respective href
		for (const directory of directories) {
			explorer_dir.push({
				text: directory.name,
				size: directory.size,
				base64: Base64.encodeURI('/' + directory.name)
			});
		}

		for (const file of files) {
			explorer_file.push({
				text: file.name,
				size: file.size
			});
		}

		if (currentFolder == '/') {
			current_directory = false;
		} else {
			current_directory = true;
		}

		console.log('explorer_dir', explorer_dir);
		console.log('explorer_file', explorer_file);
	}
</script>

{#if res}
	<div
		class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
		<div class="flex items-center mb-2">
			<h2>Filesystem of "{id}"</h2>
		</div>

		{#if current_directory}
			<div>
				<button class="flex items-center" on:click={() => changePage('Lw')}
					><Fa icon={faArrowLeft} /> &nbsp; Back</button>
			</div>
			<br />
		{/if}

		{#each explorer_dir as { text, base64, size }}
			<div class="flex justify-start ml-4 mb-2">
				{#if text == currentFolder.replace('/', '') || text == id}
					<Fa icon={faFolderOpen} />
					<div class="ml-32">
						<b> > Current Dir :</b>
						{text}
					</div>
				{:else}
					<Fa icon={faFolder} />
					<div class="ml-32">
						<button on:click={() => changePage(base64)}><b>Dir :</b> {text}</button>
					</div>
				{/if}
				<div class="ml-auto">
					Size : {size}
				</div>
			</div>
		{/each}
		{#each explorer_file as { text, size }}
			<div class="flex ml-4 mb-2">
				<Fa icon={faFile} />
				<div class="ml-32">
					<b>File</b> : {text}
				</div>

				<div class="ml-auto">
					Size : {size}
				</div>
			</div>
		{/each}
	</div>
{:else}
	Loading...
{/if}
