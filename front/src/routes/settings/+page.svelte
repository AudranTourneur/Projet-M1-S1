<script>

	import { SlideToggle } from "@skeletonlabs/skeleton";

	let value = false;

</script>
TODO

<label class="label">
	<span>Statistics capture interval (in seconds)</span>
	<input class="input" type="text" placeholder="Input" value="15" />
</label>

<label class="label">
	<span>Change admin password</span>
	<input class="input" type="text" placeholder="Input" value="" />
</label>

TOGGLE VOLUME STATS

<SlideToggle name="slide" bind:checked={value} />
