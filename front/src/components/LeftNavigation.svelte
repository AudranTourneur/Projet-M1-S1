<script lang="ts">
	import { getDrawerStore } from '@skeletonlabs/skeleton';

    const drawerStore = getDrawerStore();

    function drawerClose() {
        drawerStore.close();
    }
</script>

<nav class="list-nav p-4">
	<ul>
		<li><a on:click={drawerClose} href="/">Overview</a></li>
		<li><a on:click={drawerClose} href="/images">Images</a></li>
		<li><a on:click={drawerClose} href="/containers">Containers</a></li>
		<li><a on:click={drawerClose} href="/volumes">Volumes</a></li>
		<li><a on:click={drawerClose} href="/networks">Networks</a></li>
		<li><a on:click={drawerClose} href="/topology">Topology</a></li>
		<li><a on:click={drawerClose} href="/settings">Settings</a></li>
	</ul>
</nav>
