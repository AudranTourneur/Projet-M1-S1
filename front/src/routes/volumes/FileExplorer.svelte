<script lang="ts">
    import { onMount } from 'svelte';
    import { Base64 } from 'js-base64';

    export let id: string;
    let path = '/';
    let files: any[] = [];
    let directories: any[] = [];
    let explorer: any[] = [];
    let res;

    onMount(async () => {
        let response = await fetch(`/volumes/${id}/filesystem/${Base64.encodeURI(path)}/api`);
        res = await response.json();
        files = res.files;
        directories = res.directories;
        console.log(res);

        // Push directories to explorer array with their respective href
        for (const directory of directories) {
            explorer.push({
                text: directory,
                href: `/volumes/${id}/filesystem/${Base64.encodeURI('/', directory)}`
            });
        }

        console.log('explorer', explorer); // Move console.log inside onMount
    });
</script>

{#if res}
    <div>
        {#each explorer as { text, href }}
		<div>
			<a href={href}>{text}</a>
		</div>
        {/each}
        {#each files as file}
            <div>FILE {file}</div>
        {/each}

    </div>
{:else}
    Loading...
{/if}
