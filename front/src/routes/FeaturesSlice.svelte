<script>
	import Card from './Card.svelte'
	import CardsContainer from './CardsContainer.svelte'
</script>

<section class="relative flex flex-col items-center px-3 md:px-8">
	<CardsContainer
		class="group grid w-full flex-wrap gap-6 text-lg font-medium text-white/70 lg:grid-cols-2 lg:grid-rows-2 lg:gap-4"
	>
		<Card class="row-span-2 min-h-[20rem]" color="purple">
			<div class="flex h-full flex-col justify-end p-8 sm:p-12">
				<h2 class="mb-6 text-5xl font-bold text-white lg:text-8xl">Smooth</h2>
				<p class="max-w-[60ch]">
					Smooth transitions. Great animations. High performance. Instant input.
				</p>

				<div class="_wrapper absolute inset-0 select-none" aria-hidden="true">
					<div class="feature-image">
						<img
							src=""
							class="feature-image_inner"
							alt=""
							aria-hidden="true"
							loading="lazy"
						/>
						<img
							src=""
							class="feature-image_inner-hover"
							alt=""
							aria-hidden="true"
							loading="lazy"
						/>
					</div>
				</div>
			</div>
		</Card>
		<Card class="min-h-[20rem]" color="purple">
			<div class="flex h-full flex-col justify-end p-8 sm:p-12">
				<h2 class="mb-6 text-5xl font-bold text-white">Easy to configure</h2>
				<p class="max-w-[60ch]">
					Live reloading config. Easy plain-text format. Sensible defaults. Great documentation.
				</p>

				<div class="_wrapper absolute inset-0 select-none" aria-hidden="true">
					<div class="feature-image">
						<img
							src=""
							class="feature-image_inner"
							alt=""
							aria-hidden="true"
							loading="lazy"
						/>
						<img
							src=""
							class="feature-image_inner-hover"
							alt=""
							aria-hidden="true"
							loading="lazy"
						/>
					</div>
				</div>
			</div>
		</Card>
		<Card class="min-h-[20rem]" color="purple">
			<div class="flex h-full flex-col justify-end p-8 sm:p-12">
				<h2 class="mb-6 text-5xl font-bold text-white">Dynamic tiling</h2>
				<p class="max-w-[60ch]">
					Automatic tiling that just works. Supports multiple fine-tuneable layouts.
				</p>
				<div class="_wrapper absolute inset-0 select-none" aria-hidden="true">
					<div class="feature-image">
						<img
							src=""
							class="feature-image_inner"
							alt=""
							aria-hidden="true"
							loading="lazy"
						/>
						<img
							src=""
							class="feature-image_inner-hover"
							alt=""
							aria-hidden="true"
							loading="lazy"
						/>
					</div>
				</div>
			</div>
		</Card>
	</CardsContainer>
</section>

<style lang="postcss">
	.icon-feature {
		@apply flex items-center justify-center gap-3 font-bold text-slate-400;
	}

	.feature-image {
		position: absolute;
		inset: 0 0 0 0;
		opacity: 0.5;
		z-index: -10;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: opacity 180ms ease-in-out;

		._wrapper:hover & {
			opacity: 1;
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
		}
	}
	.feature-image_inner-hover {
		opacity: 0;
	}
	._wrapper:hover {
		& .feature-image_inner {
			opacity: 0;
		}
		& .feature-image_inner-hover {
			opacity: 1 !important;
			filter: saturate(1.3);
		}
	}

	p {
		word-break: pretty;
	}
</style>
