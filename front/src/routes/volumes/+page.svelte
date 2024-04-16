<script lang="ts">
    import type { VolumeData } from '$lib/types/VolumeData';
    import VolumeBox from './VolumeBox.svelte';

    export let data;

    const volumes: VolumeData[] = data.volumes;
    let visibleVolumes = [...volumes];
    let search = '';
    let sortBy = '';

    function searchVolumes() {
        visibleVolumes = volumes.filter(volume => 
            volume.name.toLowerCase().includes(search.toLowerCase()) &&
            (sortBy === '' || sortBy === 'date' ? true : false) // Ajoutez d'autres conditions ici pour les autres critères de tri
        );
        if (sortBy === 'date') {
            visibleVolumes.sort((a, b) => new Date(a.creationDate).getTime() - new Date(b.creationDate).getTime());
        } else if (sortBy === 'alphabetical') {
            visibleVolumes.sort((a, b) => a.name.localeCompare(b.name));
        }
    }
</script>

<h1 class="text-center text-4xl mb-5">Volumes</h1>
<div class="mx-auto max-w-xs flex items-center mb-4">
    <input bind:value={search} type="text" placeholder="Search by name" class="input mr-2" on:input={searchVolumes} />
    <select bind:value={sortBy} class="input" on:change={searchVolumes}>
        <option value="">Sort by...</option>
        <option value="date">Date</option>
        <option value="alphabetical">Alphabetical</option>
    </select>
</div>
<div class="w-full">
    {#each visibleVolumes as volume, i}
        <VolumeBox volume={volume} />
    {/each}
</div>
