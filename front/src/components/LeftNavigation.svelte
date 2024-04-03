<script lang="ts">
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	import Fa from 'svelte-fa';
	import {
		faHouse,
		faCoins,
		faCog,
		faCubes,
		faImages,
		faLayerGroup,
		faNetworkWired,
		faCircleNodes,

		faGlobe

	} from '@fortawesome/free-solid-svg-icons';

	const drawerStore = getDrawerStore();
	$: currentPath = $page.url.pathname;

	function drawerClose() {
		drawerStore.close();
	}

	const menu = [
		{ icon: faHouse, text: 'Overview', href: '/overview' },
		{ icon: faImages, text: 'Images', href: '/images' },
		{ icon: faCubes, text: 'Containers', href: '/containers' },
		{ icon: faLayerGroup, text: 'Composes', href: '/composes' },
		{ icon: faCoins, text: 'Volumes', href: '/volumes' },
		{ icon: faNetworkWired, text: 'Networks', href: '/networks' },
		{ icon: faCircleNodes, text: 'Topology', href: '/topology' },
		{ icon: faGlobe, text: 'DNS', href: '/dns' },
		{ icon: faCog, text: 'Settings', href: '/settings' }
	];
</script>

<nav class="flex gap-3 flex-col p-4">
	{#each menu as item}
		<a
			on:click={drawerClose}
			href={item.href}
			class="btn items-center justify-start"
			class:variant-soft-primary={currentPath === item.href}
			class:variant-soft={currentPath !== item.href}>
			<Fa icon={item.icon} />
			<span>{item.text}</span>
		</a>
	{/each}
</nav>
