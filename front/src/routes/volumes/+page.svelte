<script>
    import item from '../info.json';

    let volumes = item.docker_info.volumes;

    function deleteVolume(index) {
        // implémenter la fontion adéquate
        console.log(`Deleting volume with index ${index}`);
    }

    function downloadVolume(index) {
        // implémenter la fonction a
        console.log(`Downloading volume with index ${index}`);
    }
</script>

<div class="w-full">
    {#each volumes as { id, name, size }, i}
        <div class="border border-gray-300 rounded p-4 mb-4" key={i}>
            <h3 class="text-lg font-semibold mb-2">Volume Information</h3>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">ID:</span>
                <span>{id}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Name:</span>
                <span>{name}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Size:</span>
                <span>{size}</span>
            </div>
            <div class="flex justify-end items-center">
                <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => deleteVolume(i)}>
                    Delete
                </button>
                <button class="bg-blue-500 text-white px-4 py-2 rounded" on:click={() => downloadVolume(i)}>
                    Download
                </button>
            </div>
        </div>
    {/each}
</div>

