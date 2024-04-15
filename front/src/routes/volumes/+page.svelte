<script lang="ts">
    import type { VolumeData } from '$lib/types/VolumeData';
    import VolumeBox from './VolumeBox.svelte';

    export let data;

    const volumes : VolumeData[] = data.volumes;

    console.log(volumes);
</script>

<div class="w-full">
    {#each volumes as volume, i}
        <VolumeBox volume={volume} />
    {/each}
</div>
