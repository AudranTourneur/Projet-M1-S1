<script lang="ts">
	export let data;

	import { onMount } from 'svelte';

	onMount(() => {
		console.log('todo');
	});
</script>

todo {data}
