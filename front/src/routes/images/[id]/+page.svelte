<script lang="ts">
    export let data;

    console.log(data);

    let image = data;

    import { onMount } from 'svelte';

    onMount(() => {
        console.log('todo');
    });
</script>

<span>id: {image.id}  ||</span>
<span>tags: {image.tags}  ||</span>
<span>size: {image.size}  ||</span>
<span>created: {image.created}</span>