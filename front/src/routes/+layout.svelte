<script lang="ts">
	import '../app.postcss';
	import { page } from '$app/stores';
	import { AppBar, AppShell, Avatar, Drawer, LightSwitch } from '@skeletonlabs/skeleton';
	import LeftNavigation from '../components/LeftNavigation.svelte';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { getDrawerStore } from '@skeletonlabs/skeleton';

	import Fa from 'svelte-fa';
	import { faDoorOpen, faXmark } from '@fortawesome/free-solid-svg-icons';
	import { goto } from '$app/navigation';

	initializeStores();

	const drawerStore = getDrawerStore();
	let disconnectPanelVisible = false;

	function drawerOpen() {
		drawerStore.open();
	}

	function togglePanel() {
		disconnectPanelVisible = !disconnectPanelVisible;
	}

	async function disconnect() {
		const res = await fetch('/api', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ action: 'disconnect'})
		})

		const serverResponseJson = await res.json() as {
			success : boolean,
			message : string | undefined
		}

		if (serverResponseJson.success) {
			goto('/login')
		}
	}

	export let data;

	$: route = $page.route?.id;
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
				<button class="md:hidden btn btn-sm mr-2" on:click={drawerOpen}>
					<span>
						<svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
							<rect width="100" height="20" />
							<rect y="30" width="100" height="20" />
							<rect y="60" width="100" height="20" />
						</svg>
					</span>
				</button>
				<div class="bg-surface-400 rounded-container-token flex items-center py-1 px-2 gap-2">
					<img src="/logo.png" alt="OkiDocky logo" class="h-7 md:h-10" />
					<h1 class="text-xl font-bold text-black">OkiDocky</h1>
				</div>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch />
				<Avatar initials="OD" width="w-10" background="bg-primary-500" on:click={togglePanel} />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<div id="sidebar-left" class="hidden md:block">
			<LeftNavigation />
		</div>
	</svelte:fragment>

	{#if disconnectPanelVisible}
		<div class="absolute top-0 right-0 mt-16 mr-4 p-4 bg-surface-100-800-token shadow rounded">
			<br />
			<button class="btn items-center bg-red-500 text-white" on:click={disconnect}>
				<Fa icon={faDoorOpen} /><span>Log out</span></button>
			<br />
			<button class="btn items-center" on:click={togglePanel}
				><Fa icon={faXmark} /> <span>Cancel</span></button>
		</div>
	{/if}

	<!-- (sidebarRight) -->
	<!-- (pageHeader) -->
	<!-- Router Slot -->

	{#key route}
	{#if route !== '/topology'}
		<div class="container py-10 px-1 sm:px-3 md:px-6 mx-auto">
			<slot />
		</div>
	{:else}
		<slot />
	{/if}
	{/key}
	<!-- ---- / ---- -->
	<!-- (pageFooter) -->
	<!-- (footer) -->
</AppShell>
