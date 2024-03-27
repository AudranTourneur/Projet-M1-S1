<script lang="ts">
    export let data;

    console.log(data);

    let image = data;

    import { onMount } from 'svelte';

    onMount(() => {
        console.log('todo');
    });

    function formatBytes(bytes: number, decimals = 2): string {
    if (!+bytes) return '0 Bytes'

    const k = 1024
    const dm = decimals < 0 ? 0 : decimals
    const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']

    const i = Math.floor(Math.log(bytes) / Math.log(k))

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
    }

    function formatCreatedDate(sec: number): string {
        const date = new Date(sec * 1000);
        const iso = date.toISOString();
        return iso.split('.')[0].replace('T', ' ');
    }
</script>

<div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg overflow-auto">
    <div>
        <span>id: {image.id}</span>
    </div>
    <br/>
    <div>
        <span>tags: {image.tags}</span>
    </div>
    <br/>
    <div>
        <span>Created :{formatCreatedDate(image.created)}</span>
    </div>
    <br/>
    <div>
        Size :{formatBytes(image.size)}
    </div>
</div>
