<script lang="ts">
	export let tooltipText: string;

	import { flip, shift } from 'svelte-floating-ui/dom';
	import { createFloatingActions } from 'svelte-floating-ui';

	export let textSizeClass = "text-sm"

	const [floatingRef, floatingContent] = createFloatingActions({
		strategy: 'absolute',
		placement: 'bottom',
		middleware: [flip(), shift()]
	});

	let showTooltip: boolean = false;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<span
	{...$$props}
	on:mouseenter={() => (showTooltip = true)}
	on:mouseleave={() => (showTooltip = false)}
	use:floatingRef>
	<slot />
</span>
{#if showTooltip}
	<div class="bg-surface-300-600-token shadow px-0.5 rounded-container-token {textSizeClass}" use:floatingContent>
		{tooltipText}
	</div>
{/if}
