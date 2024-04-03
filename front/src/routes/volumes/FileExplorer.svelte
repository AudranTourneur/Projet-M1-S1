<script lang="ts">
	import { onMount } from 'svelte';
	import { Base64 } from 'js-base64';

	export let id: string;
	let path = '/';
	let files: any[] = [];
	let directories: any[] = [];

	let res;

	onMount(async () => {
		let response = await fetch(`/volumes/${id}/filesystem/${Base64.encodeURI(path)}/api`);
		res = await response.json();
		files = res.files;
		directories = res.directories;
		console.log(res);
	});
</script>

{#if res}
<div>
	{#each directories as directory}
		<div>DIR {directory}</div>
	{/each}
	{#each files as file}
		<div>FILE {file}</div>
	{/each}
</div>
{:else}
Loading...
{/if}
