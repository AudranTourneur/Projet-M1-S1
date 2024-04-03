<script lang="ts">
	import type { ComposeData } from '$lib/types/ComposeData';

	import yaml from 'highlight.js/lib/languages/yaml';
	import hljs from 'highlight.js/lib/core';
	import 'highlight.js/styles/github.css';

	let CodeJar;
	let highlight;

	import { onMount } from 'svelte';

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

<div>
	{compose.filePath}
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

<div class="bg-black">
	{#each lines as line}
		<span>{line}</span> <br />
	{/each}
</div>
{#each compose.containers as container}
	<div>
		{container.names.join(', ')}
	</div>
{/each}
