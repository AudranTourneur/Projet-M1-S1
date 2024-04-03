<script lang="ts">

    import ContainerBox from "./ContainerBox.svelte";

    export let data;
    let containers = data.containers;

    const refetchContainers = async () => {
        await fetch("/containers/api/list")
            .then(response => response.json())
            .then(data => {
                containers = data.containers;
            });
    }
</script>

<div class="w-full">
    {#each containers as container}
        <ContainerBox {container} refresh={refetchContainers} />
    {/each}
</div>
