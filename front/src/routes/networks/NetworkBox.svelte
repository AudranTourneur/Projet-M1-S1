<script lang="ts">
	import { Fa } from 'svelte-fa';
	import type { NetworkData } from '$lib/types/NetworkData.js';
	import { faEllipsisVertical } from '@fortawesome/free-solid-svg-icons';
	import Tooltip from '../../components/Tooltip.svelte';
	export let network: NetworkData;
</script>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4 flex justify-between items-center gap-2">
	<div class="flex flex-col justify-center ml-5">
		<div class="mb-3">
			<span class="font-bold"> Name : </span>
			{network.name}
		</div>
		<div>
			<span class="font-bold"> ID : </span>
			<Tooltip tooltipText={`ID : ${network.id}`}>
				{network.id.substring(0, 19)}...
			</Tooltip>
		</div>
		<div>
			<span class="font-bold">Created :</span>
			{new Date(network.created).toLocaleString()}
		</div>
	</div>
	<div class="flex flex-col items-center gap-2">
		<div class="flex items-center gap-10">
			<div>
				{#if network.labels && Object.keys(network.labels).length > 0}
					<div>
						<span class="font-bold">Labels :</span>
						{#each Object.entries(network.labels) as [str1, str2]}
							<div>- {str1} : {str2}</div>
						{/each}
					</div>
				{/if}
			</div>
			<div>
				{#if network.ipamConfig && Object.keys(network.ipamConfig).length > 0}
					<div>
						<span class="font-bold">Ipam Config :</span>
						{#each network.ipamConfig as config}
							<div>- Subnet: {config.subnet ? config.subnet : 'none'}</div>
							<div>- IP Range: {config.ipRange ? config.ipRange : 'none'}</div>
							<div>- Gateway: {config.gateway ? config.gateway : 'none'}</div>
							{#if config.auxAddresses}
								<div>Aux Addresses:</div>
								<ul>
									{#each Object.entries(config.auxAddresses) as [key, value]}
										<li>{key}: {value}</li>
									{/each}
								</ul>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>

	<div class="flex gap-1">
		<a href="/networks/{network.id}" class="btn variant-ghost p-2">
			<Fa icon={faEllipsisVertical} fw />
		</a>
	</div>
</div>
