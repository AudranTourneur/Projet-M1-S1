<script lang="ts">
	import type { ContainerData } from "$lib/types/ContainerData";
	import type { OurPortTypeEnum } from "$lib/types/OurPortTypeEnum";
	import type { PortData } from "$lib/types/PortData";

    export let container: ContainerData;

    const c = container;

    let ports = c.ports.filter(x => x.ip?.includes('.'))
    console.log(ports)


	function addPort(): any {
		throw new Error("Function not implemented.");
	}


	function deleteBiding() {
		throw new Error("Function not implemented.");
	}


    let newPortType: OurPortTypeEnum
	function editPortType(port: PortData): any {
        port.type = newPortType;
        ports = ports;
	}


    let newPrivatePort: number
	function editPrivatePort(port: PortData): any {
        port.privatePort = newPrivatePort;
        ports = ports;
	}


    let newPortIP = '';
    function editPortIP(port: PortData): any {
        port.ip = newPortIP;
        ports = ports;
	}


    let newPublicPort: number
	function editPublicPort(port: PortData): any {
        port.publicPort = !! newPublicPort ? newPublicPort:0;
        ports = ports;
	}
</script>

<div>
	Rebind ports
{#each ports as port}
    Host port {port.publicPort} on {port.ip} is binded to the internal port {port.privatePort} for the protocol {port.type}
        <button class="btn variant-glass-primary" on:click={() => editPublicPort(port)}>Edit public port</button>
        <input title="Input (number)" type="number" bind:value={port.publicPort} placeholder="new public port" />

        <button class="btn variant-glass-primary" on:click={() => editPortIP(port)}>Edit port IP</button>
        <input title="Input (text)" type="text" bind:value={newPortIP} placeholder="new IP" />

        <button class="btn variant-glass-primary" on:click={() => editPrivatePort(port)}>Edit private port</button>
        <input title="Input (number)" type="number" bind:value={newPrivatePort} placeholder="new private port" />

        <button class="btn variant-glass-primary" on:click={() => editPortType(port)}>Edit port type</button>
        <input title="Input (text)" type="text" bind:value={newPortType} placeholder="new port type" />

        <button class="btn variant-glass-primary" on:click={() => deleteBiding()}>delete biding</button>
        <br>
{/each}

<button class="btn variant-glass-primary" on:click={() => addPort()}>Add port</button>
</div>