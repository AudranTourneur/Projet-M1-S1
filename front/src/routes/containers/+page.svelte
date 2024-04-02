<script lang="ts">

    import ContainerBox from "./ContainerBox.svelte";

    export let data;
    const containers = data.containers;
</script>

<div class="w-full">
    normal que les containers soient dupliqués, je travaille dessus :-) Simon
    {#each containers as container}
        <ContainerBox {container} />
    {/each}
</div>
