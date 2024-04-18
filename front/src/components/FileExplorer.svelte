<script lang="ts">
	import { onMount } from 'svelte';
	import { Base64 } from 'js-base64';
	import Fa from 'svelte-fa';
	import { faFolder, faFolderOpen, faFile, faArrowLeft } from '@fortawesome/free-solid-svg-icons';
	import { formatBytes } from '$lib/FormatUtils';

	export let base64Name: string;

	console.log('base 64 name', base64Name);
	let current_directory = false;

	let path = '/';
	let currentFolder = '/';

	let files: any[] = [];
	let directories: any[] = [];

	type File = {
		name: string;
		size: string;
	};

	type Dir = File;

	let explorer_dir: Dir[] = [];
	let explorer_file: File[] = [];

	type BackendResponse = {
		currentFolder: string;
		directories: Dir[];
		files: File[];
	};

	let res: BackendResponse | null = null;

	onMount(async () => {
		const url = `/volumes/${base64Name}/filesystem/${Base64.encodeURI(path)}/api`;
		console.log(url);
		let response = await fetch(url);
		res = (await response.json()) as BackendResponse;
		update(res);
	});

	async function changePage(fileName: string) {
		console.log('File name', fileName, 'path', path);

		if (fileName === '..') {
			path = path.substring(0, path.lastIndexOf('/'));
			if (path == '') {
				path = '/';
			}
		} else {
			// remove last slash
			path = path.replace(/\/$/, '');
			path = path + '/' + fileName;
		}

		if (path.length > 1 && path.endsWith('/')) {
			path = path.substring(0, path.length - 1);
		}

		if (path == '') {
			path = '/';
		}

		const base64path = Base64.encodeURL(path);

		const urlApi = `/volumes/${base64Name}/filesystem/${base64path}/api`;
		console.log('making call to', urlApi, 'with path', path);
		let response = await fetch(urlApi);
		res = (await response.json()) as BackendResponse;
		update(res);
	}

	function update(res: BackendResponse) {
		files = res.files;
		directories = res.directories;
		currentFolder = res.currentFolder;
		explorer_dir = [];
		explorer_file = [];

		// Push directories to explorer array with their respective href
		for (const directory of directories) {
			explorer_dir.push({
				name: directory.name,
				size: directory.size
			});
		}

		for (const file of files) {
			explorer_file.push({
				name: file.name,
				size: file.size
			});
		}

		if (currentFolder == '/') {
			current_directory = false;
		} else {
			current_directory = true;
		}
	}
</script>

{#if res}
	<div
		class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
		<div class="flex items-center mb-2">
			<h2>Volume's filesystem ({path})</h2>
		</div>

		{#if current_directory}
			<div>
				<button class="flex items-center" on:click={() => changePage('..')}
					><Fa icon={faArrowLeft} /> &nbsp; Back</button>
			</div>
			<br />
		{/if}

		{#each explorer_dir as { name, size }}
			<div class="flex justify-start ml-4 mb-2">
				{#if name == currentFolder.replace('/', '')}
					<Fa icon={faFolderOpen} />
					<div class="ml-32">
						<b> > Current Dir :</b>
						{name}
					</div>
				{:else}
					<Fa icon={faFolder} />
					<div class="ml-8">
						<button class="hover:text-gray-400" on:click={() => changePage(name)}
							><b>Dir:</b>&nbsp{name}</button>
					</div>
				{/if}
				<div class="ml-auto italic">
					{formatBytes(size)}
				</div>
			</div>
		{/each}
		{#each explorer_file as { name, size }}
			<div class="flex ml-4 mb-2">
				<Fa icon={faFile} />
				<div class="ml-8">
					<b>File:</b>&nbsp{name}
				</div>

				<div class="ml-auto italic">
					{formatBytes(size)}
				</div>
			</div>
		{/each}
	</div>
{:else}
	Loading...
{/if}
