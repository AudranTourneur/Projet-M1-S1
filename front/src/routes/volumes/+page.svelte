<script lang="ts">

    export let data;

    const volumes = data.volumes;

    function deleteVolume(index: number) {
        // implémenter la fonction adéquate
        console.log(`Deleting volume with index ${index}`);
    }

    function downloadVolume(index: number) {
        // implémenter la fonction adéquate
        console.log(`Downloading volume with index ${index}`);
    }
</script>

<div class="w-full">
    {#each volumes as volume, i}
        <div class="border border-gray-300 rounded p-4 mb-4">
            <h3 class="text-lg font-semibold mb-2">Volume Information</h3>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">ID:</span>
                <span>{volume.createdAt}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Name:</span>
                <span>{volume.name}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Size:</span>
                <span>{volume.mountpoint}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Size:</span>
                <span>{volume.size}</span>
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
