<script lang="ts">
	import type { ContainerData } from "$lib/types/ContainerData";
	import type { PortData } from "$lib/types/PortData";

    export let container: ContainerData;

    const c = container;

    let ports = c.ports.filter(x => x.ip?.includes('.'))


	function addPort(): any {
		throw new Error("Function not implemented.");
	}


	function deleteBiding() {
		throw new Error("Function not implemented.");
	}


	function editPortType(): any {
		throw new Error("Function not implemented.");
	}


	function editPrivatePort(): any {
		throw new Error("Function not implemented.");
	}


    function editPortIP(): any {
		throw new Error("Function not implemented.");
	}


	function editPublicPort(port: PortData): any {
		console.log(port)
        port.publicPort: !! null ? ++ : null
        ports = ports;
	}
</script>

<div>
	Rebind ports
{#each ports as port}
    Host port {port.publicPort} on {port.ip} is binded to the internal port {port.privatePort} for the protocol {port.type}
        <button class="btn variant-glass-primary" on:click={() => editPublicPort(port)}>Edit public port</button>
        <input bind:value={port.publicPort} placeholder="enter the new port" />
        <button class="btn variant-glass-primary" on:click={() => editPortIP()}>Edit port IP</button>
        <button class="btn variant-glass-primary" on:click={() => editPrivatePort()}>Edit private port</button>
        <button class="btn variant-glass-primary" on:click={() => editPortType()}>Edit port type</button>
        <button class="btn variant-glass-primary" on:click={() => deleteBiding()}>delete biding</button>
        <br>
{/each}

<button class="btn variant-glass-primary" on:click={() => addPort()}>Add port</button>
</div>