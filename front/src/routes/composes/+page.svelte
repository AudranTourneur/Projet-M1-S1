<script lang="ts">
	import type { ComposeList } from "$lib/types/ComposeList";

    export let data: ComposeList;

    let composes = data.composes;
</script>

{#each composes as compose} 
    <div class="flex items-center">
        <a href="/composes/{compose.id}">{compose.filePath}</a>
    </div>
{/each}
