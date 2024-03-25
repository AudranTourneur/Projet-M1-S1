<script lang="ts">
    export let data;

    console.log(data);

    let image = data;

    import { onMount } from 'svelte';

    onMount(() => {
        console.log('todo');
    });
</script>


<div class="space-y-5">
            <div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg overflow-auto">
                <div>
                    <span>{image.id}</span>
                </div>
                <br/>
                <div>
                    <span>tags : {image.tags}</span>
                </div>
                <br/>
                <div>
                    Size :{formatBytes(image.size)}
                </div>
                <br/>
                <div>
                    <span>created: {image.created}  </span>
                </div>


            </div>
        </div>

<span>id: {image.id}  ||</span>
<span>tags: {image.tags}  ||</span>
<span>created: {image.created}</span>
<span>size: {image.size}  ||</span>
