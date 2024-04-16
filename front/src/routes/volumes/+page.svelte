<script lang="ts">
    import type { VolumeData } from '$lib/types/VolumeData';
    import VolumeBox from './VolumeBox.svelte';

    export let data;

    const volumes: VolumeData[] = data.volumes;
    let visibleVolumes = [...volumes];
    let search = '';


    function searchVolumes() {
        visibleVolumes = volumes.filter(volume => volume.name.toLowerCase().includes(search.toLowerCase()));
    }
</script>

<h1 class="text-center text-4xl mb-5">Volumes</h1>
<div class="mx-auto max-w-xs">
    <input bind:value={search} type="text" placeholder="Search by name" class="input mb-4" on:input={searchVolumes} />
</div>
<div class="w-full">
    {#each visibleVolumes as volume, i}
        <VolumeBox volume={volume} />
    {/each}
</div>
