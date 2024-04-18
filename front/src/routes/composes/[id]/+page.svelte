<script lang="ts">
	import type { ComposeData } from '$lib/types/ComposeData';

	import yaml from 'highlight.js/lib/languages/yaml';
	import hljs from 'highlight.js/lib/core';
	import 'highlight.js/styles/github.css';

	let CodeJar;
	let highlight;

	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { faCube } from '@fortawesome/free-solid-svg-icons';

	onMount(async () => {
		hljs.registerLanguage('yaml', yaml);

		highlight = (code, syntax) =>
			hljs.highlight(code, {
				language: syntax
			}).value;

		({ CodeJar } = await import('@novacbn/svelte-codejar'));
	});

	export let data: ComposeData;

	let compose = data;

	const lines = compose.fileContent.split('\n');

	let value = compose.fileContent;
</script>

<div
	class="flex flex-col items-center justify-center w-full border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token">
	<div class="flex justify-center items-center w-full">
		<h1
			class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-2">
			{compose.filePath}
		</h1>
	</div>
	<div class="flex">
		{#each compose.containers as c}
			<div class="flex flex-col p-8">
				<a href="/containers/{c.id}">
					<div class="flex flex-col items-center bg-surface-400/40 dark:bg-surface-800/40 border rounded-lg shadow p-4 hover:bg-surface-200/20 dark:hover:bg-surface-600/30">
						<Fa size="2x" icon={faCube}></Fa>
						<span class="text-md hover:text-gray-500">{c.names[0]}</span>
					</div>
				</a>
			</div>
		{/each}
	</div>
</div>

{#if CodeJar && highlight}
	<CodeJar
		class="hljs border rounded-3 p-3"
		syntax="yaml"
		addClosing={true}
		spellcheck={false}
		tab={'\t'}
		{highlight}
		bind:value />
{:else}
	Loading...
{/if}
