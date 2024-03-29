<script lang="ts">
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import { faHouse, faImage, faBoxOpen, faCoins, faWifi, faChessBoard, faCog } from '@fortawesome/free-solid-svg-icons';

	const drawerStore = getDrawerStore();

	function drawerClose() {
		drawerStore.close();
	}
</script>

<nav class="list-nav p-4">
	<ul class="flex gap-2 flex-col">
		<li class="flex items-center"><Fa icon={faHouse} /><a class="fa-solid fa-house" on:click={drawerClose} href="/overview"> Overview</a></li>
		<li class="flex items-center"><Fa icon={faImage} /><a class="fa-regular fa-image" on:click={drawerClose} href="/images"> Images</a></li>
		<li class="flex items-center"><Fa icon={faBoxOpen} /><a class="fa-solid fa-box-open" on:click={drawerClose} href="/containers"> Containers</a></li>
		<li class="flex items-center"><Fa icon={faCoins} /><a class="fa-solid fa-coins" on:click={drawerClose} href="/volumes">Volumes</a></li>
		<li class="flex items-center"><Fa icon={faWifi} /><a class="fa-solid fa-wifi" on:click={drawerClose} href="/networks">Networks</a></li>
		<li class="flex items-center"><Fa icon={faChessBoard} /><a class="fa-solid fa-chess-board" on:click={drawerClose} href="/topology">Topology</a></li>
		<li class="flex items-center"><Fa icon={faCog} /><a class="fa-solid fa-gear" on:click={drawerClose} href="/settings">Settings</a></li>
	</ul>
</nav>
