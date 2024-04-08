<script lang="ts">
    import type { VolumeData } from '$lib/types/VolumeData';
    import type { OurPortTypeEnum } from '$lib/types/OurPortTypeEnum';
    import type { PortData } from '$lib/types/PortData';
    import { Fa } from 'svelte-fa';
    import { faPenToSquare, faCheck, faXmark } from '@fortawesome/free-solid-svg-icons';
    import { number } from 'zod';
    import Input from 'postcss/lib/input';

    export let volume: VolumeData;

    type ExtendedPortData = {
        port: PortData;
        isEditingPublicPort: boolean;
        isEditingIPPort: boolean;
        isEditingPrivatePort: boolean;
        isEditingTypePort: boolean;
        currentlyEditingValuePublicPort: number;
        currentlyEditingValueIPPort: string;
        currentlyEditingValuePrivatePort: number;
        currentlyEditingValueTypePort: OurPortTypeEnum
    };

    let ports: ExtendedPortData[] = [];

    function addPort(): any {
        let newPort: PortData = {
            ip: '0.0.0.0',
            privatePort: 0,
            publicPort: null,
            type: 'EMPTY'
        };
        ports.push({
            port: newPort,
            isEditingPublicPort: false,
            isEditingIPPort: false,
            isEditingPrivatePort: false,
            isEditingTypePort: false,
            currentlyEditingValuePublicPort: 0,
            currentlyEditingValueIPPort: '',
            currentlyEditingValuePrivatePort: 0,
            currentlyEditingValueTypePort: 'EMPTY'
        });
    }

    function togglePublicPortEditition(p: ExtendedPortData, change: boolean) {
        if (p.isEditingPublicPort) {
            p.port.publicPort = !change ? p.currentlyEditingValuePublicPort : p.port.publicPort;
        }

        p.isEditingPublicPort = !p.isEditingPublicPort;
        p.currentlyEditingValuePublicPort = p.port.publicPort || 0;
    }

    function toggleVolumeNameEditition(v: VolumeData, change: boolean) {
        if (v.isEditingVolumeName) {
            v.name = !change ? v.currentlyEditingVolumeName : v.name;
        }

        v.isEditingVolumeName = !v.isEditingVolumeName;
        v.currentlyEditingVolumeName = v.name;
    }
</script>

{#if !volume.isEditingVolumeName}
    <div>
        Rebind ports
        {#each ports as p}
            Host port
            {#if !p.isEditingPublicPort}
                {p.port.publicPort}
            {:else}
                <input
                    title="Input (number)"
                    type="number"
                    bind:value={p.currentlyEditingValuePublicPort}
                    placeholder="new public port"
                    class="input w-40" />
            {/if}


            on

            {#if !p.isEditingIPPort}
                {p.port.ip}
            {:else}
                <input
                    title="Input (text)"
                    type="text"
                    bind:value={p.currentlyEditingValueIPPort}
                    placeholder="new IP port"
                    class="input w-40" />
            {/if}


            is binded to the internal port

            {#if !p.isEditingPrivatePort}
                {p.port.privatePort}
            {:else}
                <input
                    title="Input (number)"
                    type="number"
                    bind:value={p.currentlyEditingValuePrivatePort}
                    placeholder="new IP port"
                    class="input w-40" />
            {/if}


            for the protocol

            {#if !p.isEditingTypePort}
                {p.port.type}
            {:else}
                <select class="select w-40">
                    <option value="1">EMPTY</option>
                    <option value="2">TCP</option>
                    <option value="3">UDP</option>
                    <option value="4">SCTP</option>
                </select>
            {/if}


            <br />
        {/each}

        <button class="btn variant-glass-primary" on:click={() => addPort()}>Add port</button>
    </div>
{/if}

<div>
    Rebind volumes
    Volume name

    {#if !volume.isEditingVolumeName}
        {volume.name}
        
        <button class="btn-icon variant-filled" on:click={() => toggleVolumeNameEditition(volume, true)}>
            <Fa icon={faPenToSquare} />
        </button>
    {:else}
        <input
            title="Input (text)"
            type="text"
            bind:value={volume.currentlyEditingVolumeName}
            placeholder="new volume name"
            class="input w-40" />
            
        <button class="btn-icon variant-filled" on:click={() => toggleVolumeNameEditition(volume, true)}>
            <Fa icon={faCheck} />
        </button>
        
        <button class="btn-icon variant-filled" on:click={() => toggleVolumeNameEditition(volume, false)}>
            <Fa icon={faXmark} />
        </button>
    {/if}

    <br />
</div>