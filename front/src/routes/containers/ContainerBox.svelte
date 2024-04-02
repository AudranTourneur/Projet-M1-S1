<script lang="ts">
    import copy from 'copy-to-clipboard';
    import {Fa} from "svelte-fa";
	import {
		faCheck, faCoins,
		faCopy,
		faEllipsisVertical, faGear,
		faNetworkWired,
		faPlay, faPlug,
		faStop
	} from "@fortawesome/free-solid-svg-icons";
    import type {Container} from "$lib/types/Container";
    import Tooltip from "../../components/Tooltip.svelte";
    import {truncateString} from "../../components/truncate-string";


    export let container: Container;
    export let i: number;
    console.log('Container', container);

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

    let isIdCopied = false;
    let isNameCopied = false;
    const copyToClipboardId = () => {
        isIdCopied = true;
        setTimeout(() => isIdCopied = false, 1000);
        copy(container.id);
    }
    const copyToClipboardName = () => {
        isNameCopied = true;
        setTimeout(() => isNameCopied = false, 1000);
        copy(container.names[0].substring(1, container.names[0].length));
    }
</script>

<div class="border-token border-surface-300-600-token rounded-container-token p-4 mb-4 flex justify-between items-center">
    <div class="flex flex-col">
        <div class="font-bold copy-to-clipboard">
            {#if container.names[0].length < 15}
                {container.names[0].substring(1, container.names[0].length)}
            {:else}
                <Tooltip tooltipText={container.names[0].substring(1, container.names[0].length)}>
                    {container.names[0].substring(1, 12)}
                </Tooltip>
                <div class="hide-on-clipboard-hover">...</div>
            {/if}
            <button type="button" class="btn variant-soft" on:click={copyToClipboardName}>
                {#if isNameCopied}
                    <Fa icon={faCheck} class="text-green-500"/>
                {:else}
                    <Fa icon={faCopy}/>
                {/if}
            </button>
        </div>
        <div class="copy-to-clipboard">
            <Tooltip tooltipText={container.id}>
                {container.id.substring(0, 12)}
            </Tooltip>
            <div class="hide-on-clipboard-hover">...</div>
            <button type="button" class="btn variant-soft" on:click={copyToClipboardId}>
                {#if isIdCopied}
                    <Fa icon={faCheck} class="text-green-500"/>
                {:else}
                    <Fa icon={faCopy}/>
                {/if}
            </button>
        </div>
    </div>
	<Tooltip tooltipText={container.status}>
		{#if container.isRunning}
			<Fa icon={faGear} class="text-success-500 animate-spin text-2xl" style="animation-duration: 4s;"/>
		{:else}
			<Fa icon={faGear} class="text-error-400 text-2xl" />
		{/if}
	</Tooltip>
    <Tooltip tooltipText={`Networks: ${container.networks.length === 0 ? ' - ' : container.networks.join(', ')}`}>
        <span class="chip variant-soft gap-1">
        <Fa icon={faNetworkWired}/>
            {container.networks.length === 0 ? ' - ' : truncateString(container.networks.join(', '), 20)}
    	</span>
	</Tooltip>
    <div class="flex gap-1">
        <button class="btn variant-ghost-success p-2" disabled={container.isRunning}>
            <Fa icon={faPlay} fw/>
        </button>
        <button class="btn variant-ghost-error p-2" disabled={!container.isRunning}>
            <Fa icon={faStop} fw />
        </button>
        <a href="/containers/{container.id}" class="btn variant-ghost p-2">
            <Fa icon={faEllipsisVertical} fw/>
        </a>
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