<script lang="ts">
    export let data;

    const images = data.images;

    let visibleImages = [...images];

    let search = '';

    $: {
        visibleImages = images.filter(img => img.id.toLowerCase().includes(search.toLowerCase()) || img.tags.some(tag => tag.toLowerCase().includes(search.toLowerCase())))
    }

    function deleteVolume(index: number): void {
        // Implement the logic to delete the volume with the given index
        console.log(`Deleting volume with index ${index}`);
    }

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

<div class="flex items-center justify-center">
    <form action="#" method="get" class="p-4 flex flex-col justify-center items-center gap-2">
        <h2 class="font-bold text-lg">Search by name or ID</h2>
        <input bind:value={search} type="text" placeholder="Search..." class="bg-gray-800 p-2 border rounded-l">
    </form>
</div>

<h1 class="text-center text-4xl">Images</h1>
<ul>

    {#each visibleImages as image, i}

        <div class="space-y-5">
            <div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg">
                <div>
                    <span>{image.tags}</span>
                </div>
                <br/>
                <div>
                    <span>ID : {image.id}</span>
                </div>
                <br/>
                <div>
                    <span>Created {formatCreatedDate(image.created)}</span>
                </div>
                <br/>
                <div>
                    Size {formatBytes(image.size)}
                </div>

                <div class="flex justify-end">
                    <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => deleteVolume(i)}>Delete</button>
                </div>
            </div>
        </div>
    {/each}
</ul>
