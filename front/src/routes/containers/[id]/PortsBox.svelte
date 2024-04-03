<script lang="ts">
	import type { ContainerData } from "$lib/types/ContainerData";

    export let container: ContainerData;

    const c = container;

    const ports = c.ports.filter(x => x.ip?.includes('.'))
</script>

<div>
	Rebind ports
{#each ports as port}
    Host port {port.publicPort} on {port.ip} is binded to the internal port {port.privatePort} for the protocol {port.type} 
        <button class="btn variant-glass-primary">Edit port</button>
        <br>
{/each}

<button class="btn variant-glass-primary">Add port</button>
</div>