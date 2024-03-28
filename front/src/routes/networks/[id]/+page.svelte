<script lang="ts">
	export let data;
</script>

<h1 class="text-center text-4xl">{data.name}</h1>

<div class="space-y-5">
	<div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg overflow-auto">
		<div>
			<span>{data.name}</span>
		</div>
		<br />
		<div>
			<span>ID : {data.id}</span>
		</div>
		<br />
		<div>
			<span>Created :{data.created}</span>
		</div>
		<br />
		<div>
			Labels :
			<br />
			{#each Object.entries(data.labels) as [str1, str2]}
				{str1} : {str2}
				<br />
			{/each}
		</div>
		<br />
		<div>
			Ipam Config :
			{#each data.ipamConfig as config}
				<div>
					<p>Subnet: {config.subnet}</p>
					<p>IP Range: {config.ipRange}</p>
					<p>Gateway: {config.gateway}</p>
					{#if config.auxAddresses}
						<p>Aux Addresses:</p>
						<ul>
							{#each Object.entries(config.auxAddresses) as [key, value]}
								<li>{key}: {value}</li>
							{/each}
						</ul>
					{/if}
				</div>
			{/each}
		</div>
		<br />
		<div>
			Containers :
			{#each Object.entries(data.containers) as [name, other]}
				<div>
					<p>Name: {other.name}</p>
					<p>Endpoint Id: {other.endpointId}</p>
					<p>Mac Address: {other.macAddress}</p>
					<p>IPv4 Address: {other.ipv4Address}</p>
					<p>IPv6 Address: {other.ipv6Address}</p>
					<br />
				</div>
			{/each}
		</div>
	</div>
</div>
