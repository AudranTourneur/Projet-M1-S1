<script lang="ts">
	import type { ContainerData } from '$lib/types/ContainerData';
	import type { OurPortTypeEnum } from '$lib/types/OurPortTypeEnum';
	import type { PortData } from '$lib/types/PortData';
	import { Fa } from 'svelte-fa';
	import { faPenToSquare, faCheck, faXmark } from '@fortawesome/free-solid-svg-icons';
	import { map, number } from 'zod';
	import Input from 'postcss/lib/input';
	import type { ContainerPortRebindRequest} from '$lib/types/ContainerPortRebindRequest';
	import type { ContainerPortRebind } from '$lib/types/ContainerPortRebind';

	export let container: ContainerData;

	const c = container;

	type ExtendedPortData = {
		port: PortData;
		isEditingPublicPort: boolean;
		isEditingIPPort: boolean;
		isEditingPrivatePort: boolean;
		isEditingTypePort: boolean;
		currentlyEditingValuePublicPort: number;
		currentlyEditingValueIPPort: string;
		currentlyEditingValuePrivatePort: number;
		currentlyEditingValueTypePort: OurPortTypeEnum;
	};

	let ports: ExtendedPortData[] = c.ports
		.filter((x) => x.ip?.includes('.'))
		.map((portData) => ({
			port: portData,
			isEditingPublicPort: false,
			isEditingIPPort: false,
			isEditingPrivatePort: false,
			isEditingTypePort: false,
			currentlyEditingValuePublicPort: portData.publicPort || 0,
			currentlyEditingValueIPPort: portData.ip!,
			currentlyEditingValuePrivatePort: portData.privatePort,
			currentlyEditingValueTypePort: portData.type || 'EMPTY'
		}));

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
		ports = ports;
		console.log(ports);
	}

	function togglePublicPortEditition(p: ExtendedPortData, change: boolean) {
		if (p.isEditingPublicPort) {
			p.port.publicPort = !change ? p.currentlyEditingValuePublicPort : p.port.publicPort;
		}

		p.isEditingPublicPort = !p.isEditingPublicPort;
		p.currentlyEditingValuePublicPort = p.port.publicPort || 0;

		ports = ports;
	}

	function toggleIPPortEditition(p: ExtendedPortData, change: boolean) {
		if (p.isEditingIPPort) {
			p.port.ip = !change ? p.currentlyEditingValueIPPort : p.port.ip;
		}

		p.isEditingIPPort = !p.isEditingIPPort;
		p.currentlyEditingValueIPPort = p.port.ip || '';

		ports = ports;
		console.log(ports);
	}

	function togglePrivetPortEdition(p: ExtendedPortData, change: boolean) {
		if (p.isEditingPrivatePort) {
			p.port.privatePort = !change ? p.currentlyEditingValuePrivatePort : p.port.privatePort;
		}

		p.isEditingPrivatePort = !p.isEditingPrivatePort;
		p.currentlyEditingValuePrivatePort = p.port.privatePort;

		ports = ports;
		console.log(ports);
	}

	function toggleTypePortEdition(p: ExtendedPortData, change: boolean) {
		if (p.isEditingTypePort) {
			p.port.type = !change ? p.currentlyEditingValueTypePort : p.port.type;
		}

		p.isEditingTypePort = !p.isEditingTypePort;
		p.currentlyEditingValueTypePort = p.port.type!;

		ports = ports;
		console.log(ports);
	}

	let response = [{ text: `EMPTY` }, { text: `TCP` }, { text: `UDP` }, { text: `SCTP` }];




	async function submit() {
		const newPorts: ContainerPortRebind[] = ports.map(x=>{

			return {host: x.port.privatePort , internal: x.port.publicPort || 0 , ip: x.port.ip || '' , protocol: x.port.type || ''}
		})
		const portFinal: ContainerPortRebindRequest = {ports: newPorts}
		

		const res = await fetch('/containers/' + container.id + '/api/rebind', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(portFinal)
		});
		console.log(portFinal);
	}
</script>

<!--
<table class="table">
    <thead>
        <tr>
            <th class="align-center">Rebind ports</th>
            <th class="align-center">Host port</th>
            <th class="align-center">on</th>
            <th class="align-center">is binded to the internal port</th>
            <th class="align-center">for the protocol</th>
        </tr>
    </thead>
    <tbody>
        {#each ports as p}
            <tr>
                <td class="align-center" colspan="5">
                    {#if !p.isEditingPublicPort}
                        {p.port.publicPort}
                    {:else}
                        <input
                            title="Input (number)"
                            type="number"
                            bind:value={p.currentlyEditingValuePublicPort}
                            placeholder="New public port"
                            class="input w-40" />
                    {/if}

                    {#if !p.isEditingPublicPort}
                        <button class="btn-icon variant-filled" on:click={() => togglePublicPortEditition(p, false)}>
                            <Fa icon={faPenToSquare} />
                        </button>
                    {:else}
                        <button class="btn-icon variant-filled" on:click={() => togglePublicPortEditition(p, false)}>
                            <Fa icon={faCheck} />
                        </button>

                        <button class="btn-icon variant-filled" on:click={() => togglePublicPortEditition(p, true)}>
                            <Fa icon={faXmark} />
                        </button>
                    {/if}

                    on

                    {#if !p.isEditingIPPort}
                        {p.port.ip}
                    {:else}
                        <input
                            title="Input (text)"
                            type="text"
                            bind:value={p.currentlyEditingValueIPPort}
                            placeholder="New IP port"
                            class="input w-40" />
                    {/if}

                    {#if !p.isEditingIPPort}
                        <button class="btn-icon variant-filled" on:click={() => toggleIPPortEditition(p, false)}>
                            <Fa icon={faPenToSquare} />
                        </button>
                    {:else}
                        <button class="btn-icon variant-filled" on:click={() => toggleIPPortEditition(p, false)}>
                            <Fa icon={faCheck} />
                        </button>

                        <button class="btn-icon variant-filled" on:click={() => toggleIPPortEditition(p, true)}>
                            <Fa icon={faXmark} />
                        </button>
                    {/if}

                    {#if !p.isEditingPrivatePort}
                        {p.port.privatePort}
                    {:else}
                        <input
                            title="Input (number)"
                            type="number"
                            bind:value={p.currentlyEditingValuePrivatePort}
                            placeholder="New IP port"
                            class="input w-40" />
                    {/if}

                    {#if !p.isEditingPrivatePort}
                        <button class="btn-icon variant-filled" on:click={() => togglePrivetPortEdition(p, false)}>
                            <Fa icon={faPenToSquare} />
                        </button>
                    {:else}
                        <button class="btn-icon variant-filled" on:click={() => togglePrivetPortEdition(p, false)}>
                            <Fa icon={faCheck} />
                        </button>

                        <button class="btn-icon variant-filled" on:click={() => togglePrivetPortEdition(p, true)}>
                            <Fa icon={faXmark} />
                        </button>
                    {/if}

                    {#if !p.isEditingTypePort}
                        {p.port.type}
                    {:else}
                        <select bind:value={p.currentlyEditingValueTypePort} class="select w-40">
                            {#each response as response}
                                <option value={response.text}>
                                    {response.text}
                                </option>
                            {/each}
                        </select>
                    {/if}

                    {#if !p.isEditingTypePort}
                        <button class="btn-icon variant-filled" on:click={() => toggleTypePortEdition(p, false)}>
                            <Fa icon={faPenToSquare} />
                        </button>
                    {:else}
                        <button class="btn-icon variant-filled" on:click={() => toggleTypePortEdition(p, false)}>
                            <Fa icon={faCheck} />
                        </button>

                        <button class="btn-icon variant-filled" on:click={() => toggleTypePortEdition(p, true)}>
                            <Fa icon={faXmark} />
                        </button>
                    {/if}
                </td>
            </tr>
        {/each}
    </tbody>
</table>
<button class="btn variant-glass-primary" on:click={() => addPort()}>Add port</button>
<button class="btn variant-glass-primary" on:click={() => submit()}>Submit</button>
-->

