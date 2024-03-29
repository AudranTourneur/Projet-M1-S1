<script lang="ts">
    import {Fa} from "svelte-fa";
    import {faCopy} from "@fortawesome/free-solid-svg-icons";
    import type {Container} from "$lib/types/Container";
    import Tooltip from "../../components/Tooltip.svelte";

    export let container: Container;
    export let i: number;

    function deleteContainer(index: number) {
        // implémenter la fonction adéquate
        console.log(`Deleting container with index ${index}`);
    }

    function downloadContainer(index: number) {
        // implémenter la fonction adéquate
        console.log(`Downloading container with index ${index}`);
    }

    function startContainer(index: number) {
        // implémenter la fonction adéquate
        console.log(`Start a container with index ${index}`);
    }

    function stopContainer(index: number) {
        //implémenter la fonction adéquate
        console.log(`Stop a container with index ${index}`);
    }
</script>

<div class="border-token border-surface-300-600-token rounded-container-token p-4 mb-4">
    <div class="flex flex-col">
                <span class="font-bold copy-to-clipboard">
                    {#if container.names[0].length < 15}
                        {container.names[0].substring(1, container.names[0].length)}
                    {:else}
                        <Tooltip tooltipText={container.names[0].substring(1, container.names[0].length)}>
                            {container.names[0].substring(1, 12)}
                        </Tooltip>
                        <div class="hide-on-clipboard-hover">...</div>
                    {/if}
                    <button type="button" class="btn variant-soft">
                        <Fa icon={faCopy}/>
                    </button>
                </span>
        <div class="copy-to-clipboard">
            <Tooltip tooltipText={container.id}>
                {container.id.substring(0, 12)}
            </Tooltip>
            <div class="hide-on-clipboard-hover">...</div>
            <button type="button" class="btn variant-soft">
                <Fa icon={faCopy}/>
            </button>
        </div>
    </div>
</div>
<div class="border-token border-surface-300-600-token rounded-container-token p-4 mb-4">
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">ID:</span>
        <span>{container.id}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Name:</span>
        <span>{container.names}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Image ID :</span>
        <span>{container.image}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Status :</span>
        <span>{container.status}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Date of creation :</span>
        <span>{container.network}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        {#each container.ports as port}
            <span>IP: {port.ip}</span>
            <span>PrivatePort: {port.privatePort}</span>
            <span>PublicPort: {port.publicPort}</span>
            <span>Type: {port.type}</span>
        {/each}
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Ports used :</span>
        <span>{container.volumes}</span>
    </div>
    <div class="flex justify-end items-center">
        <a href="/containers/{container.id}">
            <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2">
                Info
            </button>
        </a>
        <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => deleteContainer(i)}>
            Delete
        </button>
        <button class="bg-green-500 text-white px-4 py-2 rounded mr-2" on:click={() => downloadContainer(i)}>
            Download
        </button>
        <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => stopContainer(i)}>
            Stop
        </button>
        <button class="bg-blue-500 text-white px-4 py-2 rounded" on:click={() => startContainer(i)}>
            Start
        </button>
    </div>
</div>