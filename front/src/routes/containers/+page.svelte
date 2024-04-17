<script lang="ts">
    import ContainerBox from "./ContainerBox.svelte";
    import Fa from 'svelte-fa';
    import { faCircleNotch } from '@fortawesome/free-solid-svg-icons';
    import { onMount } from 'svelte';

    export let data;
    let containers = data.containers;

    const refetchInterval = 10000;
    let interval: number;

    const refetchContainers = async () => {
        await fetch("/containers/api/list")
            .then(response => response.json())
            .then(data => {
                containers = data.containers;
            });
    }

    onMount(()=>{
        if (interval) clearInterval(interval);
        interval = setInterval(refetchContainers, refetchInterval);
    } );

</script>

<div class="flex items-center justify-center gap-2 text-surface-400-500-token text-sm">
    <Fa icon={faCircleNotch} spin size="xs" />
    Live data, updated every 10 seconds
</div>
{#key containers}
    {#each containers as container}
        <ContainerBox {container} refresh={refetchContainers} />
    {/each}
{/key}
