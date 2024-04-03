<script lang="ts">
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import { faHouse, faImage, faBoxOpen, faCoins, faWifi, faChessBoard, faCog, faCubes, faCube, faImages, faShareNodes, faLayerGroup, faGlobe } from '@fortawesome/free-solid-svg-icons';

	const drawerStore = getDrawerStore();

	function drawerClose() {
		drawerStore.close();
	}
</script>

<nav class="list-nav p-4">
	<ul class="flex gap-2 flex-col">
		<li class="flex items-center"><Fa icon={faHouse} class="w-5" /><a on:click={drawerClose} href="/overview">Overview</a></li>
		<li class="flex items-center"><Fa icon={faImages} class="w-5" /><a on:click={drawerClose} href="/images">Images</a></li>
		<li class="flex items-center"><Fa icon={faCube} class="w-5" /><a on:click={drawerClose} href="/containers">Containers</a></li>
		<li class="flex items-center"><Fa icon={faLayerGroup} class="w-5" /><a on:click={drawerClose} href="/composes">Composes</a></li>
		<li class="flex items-center"><Fa icon={faCoins} class="w-5" /><a on:click={drawerClose} href="/volumes">Volumes</a></li>
		<li class="flex items-center"><Fa icon={faWifi} class="w-5" /><a on:click={drawerClose} href="/networks">Networks</a></li>
		<li class="flex items-center"><Fa icon={faShareNodes} class="w-5" /><a on:click={drawerClose} href="/topology">Topology</a></li>
		<li class="flex items-center"><Fa icon={faGlobe} class="w-5" /><a on:click={drawerClose} href="/dns">DNS</a></li>
		<li class="flex items-center"><Fa icon={faCog} class="w-5" /><a on:click={drawerClose} href="/settings">Settings</a></li>
	</ul>
</nav>
