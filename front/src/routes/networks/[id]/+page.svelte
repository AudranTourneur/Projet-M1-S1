
<script lang="ts">
	import { faCube } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
 
 
	export let data;
 </script>
 
 
 <style>
	.box-wrapper {
		padding: 1rem; 
		margin: 1rem; 
		border-radius: 8px;
		border: 1px solid #fff;
	}
 
 
	.item {
		padding: 0.5rem;
	}
 
 
	.grid-cols-1 {
		display: grid;
		grid-template-columns: 1fr;
		grid-gap: 1rem;
	}
 
 
	.grid-cols-2 {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		grid-gap: 1rem;
	}
 
 
	.underline {
		text-decoration: underline;
	}
 </style>
 
 
 <h1 class="text-center text-4xl">{data.name}</h1>
 
 
 <div class="space-y-5">
	<div class="box-wrapper relative overflow-auto">
		<div class="{Object.keys(data).length > 2 ? 'grid-cols-2' : 'grid-cols-1'}">
			{#if data.id}
				<div class="item title">
					<span class="underline">ID :</span> {data.id}
				</div>
			{/if}
			{#if data.created}
				<div class="item title">
					<span class="underline">Created :</span><br>
					<span>{data.created}</span>
				</div>
			{/if}
			<div class="item title">
				<span class="underline">Labels :</span>
				{#each Object.entries(data.labels) as [str1, str2]}
					<div>{str1} : {str2}</div>
				{/each}
			</div>
			{#if data.ipamConfig && data.ipamConfig.length > 0}
				<div class="item title">
					<span class="underline">Ipam Config :</span>
					{#each data.ipamConfig as config}
						{#if Object.keys(config).length > 0}
							<div>
								{#each Object.entries(config) as [key, value]}
									{#if value}
										<p>{key}: {value}</p>
									{/if}
								{/each}
							</div>
						{/if}
					{/each}
				</div>
			{/if}
		</div>
	</div>
 
 
	<div class="mt-5">
		Containers :
		<div class="grid grid-cols-4 gap-4">
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
	</div>
 </div>
 