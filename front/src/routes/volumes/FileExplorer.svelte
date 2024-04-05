<script lang="ts">
	import { onMount } from 'svelte';
	import { Base64 } from 'js-base64';

	export let id: string;
	let path = '/';
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
        if (path ==  Base64.encodeURI('/' + id)){
            path = '/';
        }
		console.log('PATH SENT TO CHANGE PAGE' + path);
		const urlApi = `/volumes/${id}/filesystem/${path}/api`;
		console.log('making call to', urlApi);
		let response = await fetch(urlApi);
		console.log('??????????????????????????????????????????????????????????' + response);
		res = await response.json();
		update(res);
	}

	function update(res) {
		files = res.files;
		directories = res.directories;
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

		console.log('explorer_dir', explorer_dir);
		console.log('explorer_file', explorer_file);
	}
</script>

{#if res}
	<div>
        <div>
            <button on:click={() => changePage("Lw")}>BACK</button>
        </div>
		{#each explorer_dir as { text, base64, size }}
			<div>
				<button on:click={() => changePage(base64)}>{text}</button>
				<br />
				<p>Size : {size}</p>
			</div>
		{/each}
		{#each explorer_file as { text, size }}
			<div>FILE {text}</div>
			<br />
			<p>Size : {size}</p>
		{/each}
	</div>
{:else}
	Loading...
{/if}
