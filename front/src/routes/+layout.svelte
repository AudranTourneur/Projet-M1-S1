<script lang="ts">
	import '../app.postcss';
	import { page } from '$app/stores';
	import { AppBar, AppShell, Avatar, Drawer, LightSwitch } from '@skeletonlabs/skeleton';
	import LeftNavigation from '../components/LeftNavigation.svelte';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { getDrawerStore } from '@skeletonlabs/skeleton';

	initializeStores();

	const drawerStore = getDrawerStore();

	function drawerOpen() {
		drawerStore.open();
	}

	export let data;

	console.log('data layout', data.test)
</script>

<svelte:head>
	<title>{$page.data.metaTitle ? `${$page.data.metaTitle} | OkiDocky` : 'OkiDocky'}</title>
</svelte:head>

<Drawer>
	<LeftNavigation />
</Drawer>

<AppShell slotSidebarLeft="w-0 md:w-52 bg-surface-500/10">
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<button class="md:hidden btn btn-sm mr-4" on:click={drawerOpen}>
					<span>
						<svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
							<rect width="100" height="20" />
							<rect y="30" width="100" height="20" />
							<rect y="60" width="100" height="20" />
						</svg>
					</span>
				</button>
				<strong class="text-xl font-bold">OkiDocky</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch />
				<Avatar initials="OD" width="w-10" background="bg-primary-500" />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<div id="sidebar-left" class="hidden md:block">
			<LeftNavigation />
		</div>
	</svelte:fragment>
	<!-- (sidebarRight) -->
	<!-- (pageHeader) -->
	<!-- Router Slot -->
	<div class="container py-10 px-1 sm:px-3 md:px-6 mx-auto">
		<slot />
	</div>
	<!-- ---- / ---- -->
	<!-- (pageFooter) -->
	<!-- (footer) -->
</AppShell>
