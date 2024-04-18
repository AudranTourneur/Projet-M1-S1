<script lang="ts">
	import { Fa } from 'svelte-fa';
	import type { NetworkData } from '$lib/types/NetworkData.js';
	import { faEllipsisVertical } from '@fortawesome/free-solid-svg-icons';
	import Tooltip from '../../components/Tooltip.svelte';
	export let network: NetworkData;
	console.log(network.labels);
</script>

<div
	class="flex justify-between items-center gap-2 p-2 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border-surface-300-600-token"
    >
	<div class="flex items-center gap-2">
        <div class="flex flex-col">
            <div> 
                <span class = "font-bold"> Name : </span> {network.name}
            </div>
        </div>
		</div>
		<div class="flex flex-col">
            <div>
                <span class = "font-bold"> ID : </span>
				<Tooltip tooltipText={`ID : ${network.id}`}>
					{network.id.substring(0, 19)}...
				</Tooltip>
			</div>
			<div>
				<span class="font-bold">Created :</span>
				{new Date(network.created).toLocaleString()}
			</div>
			<div>
				{#if Object.keys(network.labels).length > 0}
					<span class="font-bold">Labels :</span>
					{#each Object.entries(network.labels) as [str1, str2]}
						<div> - {str1} : {str2}</div>
					{/each}
				{/if}
			</div>

			<div>
				{#if Object.keys(network.ipamConfig).length > 0}
					<span class="font-bold">Ipam Config :</span>
					{#each network.ipamConfig as config}
						<div> - Subnet: {config.subnet ? config.subnet : 'none'}</div>
						<div> - IP Range: {config.ipRange ? config.ipRange : 'none'}</div>
						<div> - Gateway: {config.gateway ? config.gateway : 'none'}</div>
						{#if config.auxAddresses}
							<div>Aux Addresses:</div>
							<ul>
								{#each Object.entries(config.auxAddresses) as [key, value]}
									<li>{key}: {value}</li>
								{/each}
							</ul>
						{/if}
					{/each}
				{/if}
			</div>
		</div>

		<div class="flex gap-1">
			<a href="/networks/{network.id}" class="btn variant-ghost p-2">
				<Fa icon={faEllipsisVertical} fw />
			</a>
		</div>
	</div>


<!-- <div
	class="p-3 rounded-container-token overflow-auto bg-surface-300/30 dark:bg-surface-600/30 shadow border-token border-surface-300-600-token mt-4">
	<div class="font-bold text-lg mb-2">{network.name ? network.name : 'none'}</div>
	<div class="flex justify-between items-start gap-2">
		{#if network.id}
			<div class="flex flex-col">
				<div class="text-surface-600-300-token">
					<b>ID :</b>
					<span
						class="cursor-pointer"
						title={network.id}
						on:click={() => navigator.clipboard.writeText(network.id)}>
						{network.id.substring(0, 8)}...
					</span>
				</div>
			</div>
		{/if}
		{#if network.created}
			<div class="flex flex-col">
				<div class="text-surface-600-300-token">
					<b>Created :</b>
					<span
						class="cursor-pointer"
						title={network.created}
						on:click={() => navigator.clipboard.writeText(network.created)}>
						{network.created.substring(0, 10)}...
					</span>
				</div>
			</div>
		{/if}
		{#if network.labels}
			<div class="flex flex-col">
				<div>
					<b>Labels :</b>
					<br />
					{#each Object.entries(network.labels) as [str1, str2]}
						<div>{str1} : {str2}</div>
					{/each}
				</div>
			</div>
		{/if}
		
	</div>
	<div class="flex justify-end mt-3">
		<a href="/networks/{network.id}">
			<button class="bg-blue-500 text-white px-4 py-2 rounded mr-2"> Info </button>
		</a>
	</div>
</div>
 -->
