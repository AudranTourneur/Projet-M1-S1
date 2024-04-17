<script>
	import clsx from 'clsx'
	import { getContext, onMount } from 'svelte'
	import { cardsContext } from './CardsContainer.svelte'
	import { spring } from 'svelte/motion'
	import { getIsMobile } from './getIsMobile.js'
	const { mouseCoordinates$, isHoverCards } = getContext(cardsContext)

	/** @type HTMLDivElement */
	let container
	let isMobile = false

	const damping = 0.2

	const fillX = spring(0, { damping, stiffness: 0.021, precision: 0.3 })
	const fillY = spring(0, { damping, stiffness: 0.021, precision: 0.3 })

	const bounceBack = 2
	const soft = 0.8

	let isMouseOver = false
	/** Has the mouse entered and not left*/
	let hasMouseEntered = false

	$: {
		if (container && $mouseCoordinates$?.x !== undefined) {
			updateGradient()
		}
	}

	onMount(() => {
		isMobile = getIsMobile()
	})

	function updateGradient() {
		if (isMobile) return

		const { x: rectX, y: rectY, width, height } = container.getBoundingClientRect()

		const normX = $mouseCoordinates$.x - rectX
		const normY = $mouseCoordinates$.y - rectY

		if (!isMouseOver) {
			hasMouseEntered = false

			return
		}

		if ($mouseCoordinates$.x < rectX) fillX.set(rectX + bounceBack, { soft })
		else if ($mouseCoordinates$.x > rectX + width) fillX.set(rectX + width - bounceBack, { soft })
		else fillX.set(normX)

		if ($mouseCoordinates$.y < rectY) fillY.set(rectY + bounceBack, { soft: 1 })
		if ($mouseCoordinates$.y > rectY + height) fillX.set(rectY - height - bounceBack, { soft })
		else fillY.set(normY)
	}
</script>

<div
	class={clsx('card group ', $$restProps.class)}
	style:--x={$fillX}
	style:--y={$fillY}
	class:isHoverCards={$isHoverCards}
	bind:this={container}
	on:mouseenter={() => (isMouseOver = true)}
	on:mouseleave={() => {
		isMouseOver = false
		updateGradient()
	}}
	role="contentinfo"
	on:mouseenter
	on:mouseleave
>
	<div class="z-10 h-full w-full">
		<slot>Nothing in the slot here</slot>
	</div>
	<div class="gradient" />
	<div class="gradient_black" />
</div>

<style lang="postcss">
	.card {
		@apply relative flex items-end justify-end rounded-container-token transition-colors duration-300 bg-surface-300/30 dark:bg-surface-600/30;
		z-index: 2;
		contain: paint style layout;

		/* The card border   */
		@media screen(md) {
			background: theme(colors.slate.800);

			&:hover {
				background: theme(colors.primary.900);
			}
		}
	}

	.gradient_black {
		@apply absolute inset-[2px];
		z-index: 2;
		border-radius: inherit;
		contain: strict;
		background: black;
	}

	.gradient {
		@apply pointer-events-none absolute inset-0 h-full w-full;
		width: 100%;
		border-radius: inherit;
		height: 100%;
		margin: 0px;
		z-index: 20;
		mix-blend-mode: hard-light;
		contain: strict;

		/* Gradient blob */
		&::before {
			position: absolute;
			z-index: 20;
			border-radius: inherit;
			min-width: 200%;
			min-height: 200%;
			aspect-ratio: 1 / 1;
			translate: -25% 0%;
			transform-origin: top left;
			left: 50%;
			opacity: 50%;
			transition: all 120ms ease-in-out;
			content: '';
			pointer-events: none;
			opacity: 0%;
			contain: strict;

			background:
				radial-gradient(
					ellipse at calc(var(--x) * 1px) calc(var(--y) * 1px),
					var(--color1, theme(colors.primary.500 / 80%)),
					var(--color2, theme(colors.primary.700 / 50%)) 5%,
					var(--color3, theme(colors.primary.900 / 30%)) 50%
				);

			.group:hover & {
				opacity: 100%;
				transition: all 820ms;
			}
		}
	}
</style>
