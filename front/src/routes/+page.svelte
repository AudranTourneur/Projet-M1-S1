<script lang="ts">
	import Card from './Card.svelte'
	import CardsContainer from './CardsContainer.svelte'
	import Fa from 'svelte-fa';
	import { faCoins, faCubes, faImages, faNetworkWired } from '@fortawesome/free-solid-svg-icons';

	export let data;

	let overview = data;
</script>
<CardsContainer
	class="group grid w-full gap-4 text-lg font-medium text-white/70 sm:grid-cols-2 lg:grid-cols-4 lg:grid-rows-3"
>
	<Card class="row-span-2 col-span-2 min-h-[20rem]">
		<div class="flex h-full flex-col justify-end p-8 sm:p-12">
			<h2 class="mb-6 text-4xl font-bold text-white lg:text-6xl">OkiDocky</h2>
			<p class="max-w-[60ch]">
				Okydocky revolutionizes server management with its intuitive web interface, offering seamless control over containers. Say farewell to complex command lines as Okydocky empowers users to effortlessly deploy, monitor, and scale containers with just a few clicks.
			</p>
			<div class="_wrapper absolute inset-0 select-none">
			<div class="feature-image">
				<img
					src="/logo.png"
					alt="Okydocky logo"
					loading="lazy"
				/>
			</div>
			</div>
		</div>
	</Card>
	<Card class="min-h-[20rem] col-span-2">
		<div class="flex h-full flex-col justify-end p-8 sm:p-12">
			<h2 class="text-4xl font-bold text-white">Docker {overview.versionDocker}</h2>
			<p class="max-w-[60ch]">
				Installed on the host machine
			</p>
			<div class="_wrapper absolute inset-0 select-none">
				<div class="feature-image">
					<img
						src="/logo-docker.png"
						alt="Docker logo"
						loading="lazy"
					/>
				</div>
			</div>
		</div>
	</Card>
	<Card class="min-h-[20rem] col-span-2">
		<div class="flex h-full flex-col justify-end p-8 sm:p-12">
			<h2 class="text-3xl font-bold text-white">Linux kernel {overview.versionLinux}</h2>
			<p class="max-w-[60ch]">
				In use by the host machine
			</p>
			<div class="_wrapper absolute inset-0 select-none">
				<div class="feature-image">
					<img
						src="/logo-linux.png"
						class="small"
						alt="Linux logo"
						loading="lazy"
					/>
				</div>
			</div>
		</div>
	</Card>
	{#each [
		{ title: 'Images', value: overview.images, icon: faImages },
		{ title: 'Containers', value: overview.containers, icon: faCubes },
		{ title: 'Volumes', value: overview.volumes, icon: faCoins },
		{ title: 'Networks', value: overview.networks, icon: faNetworkWired },
	] as { title, value, icon }}
		<Card class="min-h-[20rem] max-sm:col-span-2">
			<div class="flex h-full flex-col justify-end p-8 sm:p-12">
				<h2 class="text-6xl font-bold text-white md:text-8xl">{value}</h2>
				<p class="text-xl">
					{title}
				</p>
				<div class="_wrapper absolute inset-0 select-none">
					<div class="feature-image">
						<Fa
							{icon}
							size="6x"
						/>
					</div>
				</div>
			</div>
		</Card>
	{/each}
</CardsContainer>

<style lang="postcss">
    .feature-image {
        position: absolute;
        inset: 0 0 0 0;
        opacity: .3;
        z-index: -10;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: opacity 500ms ease-in-out;

        ._wrapper:hover & {
            opacity: .6;
        }

        & img {
            position: absolute;
            transition: opacity 1500ms ease-in-out;
            pointer-events: none;
            width: 400px;
            aspect-ratio: 1;
            right: -80px;
            top: 50%;
            translate: 0px -50%;

            @media screen(md) {
                width: 600px;
                right: -80px;
            }

						&.small {
							width: 400px;
							right: -40px;
            }
        }
    }
</style>


<div class="w-full mt-10">
	<div class="border border-gray-300 rounded p-4 mb-4">
		<h3 class="text-lg text-center font-semibold mb-2">Overview</h3>
		<div class="flex justify-between items-center mb-2">
			<span class="font-bold">versionDocker:</span>
			<span>{overview.versionDocker}</span>
		</div>
		<div class="flex justify-between items-center mb-2">
			<span class="font-bold">versionLinux:</span>
			<span>{overview.versionLinux}</span>
		</div>
		<div class="flex justify-between items-center mb-2">
			<span class="font-bold">images:</span>
			<span>{overview.images}</span>
		</div>
		<div class="flex justify-between items-center mb-2">
			<span class="font-bold">containers:</span>
			<span>{overview.containers}</span>
		</div>

		<div class="flex justify-between items-center mb-2">
			<span class="font-bold">volumes:</span>
			<span>{overview.volumes}</span>
		</div>

		<div class="flex justify-between items-center mb-2">
			<span class="font-bold">networks:</span>
			<span>{overview.networks}</span>
		</div>
	</div>
</div>

