<script lang="ts">
	import { faCube, faArrowLeft } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let data;
</script>

<div class="flex gap-2 items-center mb-5">
	<a href="/networks" class="btn btn-sm variant-soft">
		<Fa icon={faArrowLeft} fw class="mr-1" />
		Back to networks
	</a>
</div>

<div
	class="border-token border-surface-300-600-token bg-surface-300/30 dark:bg-surface-600/30 shadow rounded-container-token p-3 mb-4">
	<h1 class="text-center text-4xl mb-10">"{data.name}"</h1>

	<div>
		<span class="font-bold">ID : </span>
		{data.id}
	</div>
	<div>
		<span class="font-bold">Created : </span>
		{data.created}
	</div>
	<div>
		{#if data.labels && Object.keys(data.labels).length > 0}
			<span class="font-bold">Labels : </span>
			{#each Object.entries(data.labels) as [str1, str2]}
				<div>- {str1} : {str2}</div>
			{/each}
		{/if}
	</div>
	<div>
		{#if data.ipamConfig && data.ipamConfig.length > 0}
			<span class="font-bold">Ipam Config : </span>
			{#each data.ipamConfig as config}
				{#if Object.keys(config).length > 0}
					{#each Object.entries(config) as [key, value]}
						{#if value}
							<div>- {key} : {value}</div>
						{/if}
					{/each}
				{/if}
			{/each}
		{/if}
	</div>
</div>



<div class="space-y-5">
	<div class="mt-10">
		{#if data.containers && Object.keys(data.containers).length >0}
		<div class="text-xl">Containers linked to this network :</div>
		<div class="grid grid-cols-4 gap-4 mt-5">
			{#each Object.entries(data.containers) as [id, c]}
				<div class="flex items-center gap-4 border border-1 p-2 px-4 rounded-lg">
					<Fa icon={faCube} size="2.5x"></Fa>
					<div class="flex flex-col">
						<a href="/containers/{id}" class="text-xl font-semibold hover:text-surface-500">{c.name}</a>
						<span class="text-surface-600-300-token">{c.ipv4Address}</span>
						<span class="text-surface-600-300-token">{c.macAddress}</span>
					</div>
					<br />
				</div>
			{/each}
		</div>
		{:else}
		<div class="text-xl">No containers linked to this network</div>
		{/if}
	</div>
</div>
