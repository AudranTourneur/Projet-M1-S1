<script lang="ts">
    export let data;

    const containers = data.containers;


    function deleteContainer(index: number) {
        // implémenter la fonction adéquate
        console.log(`Deleting container with index ${index}`);
    }

    function downloadContainer(index: number) {
        // implémenter la fonction adéquate
        console.log(`Downloading container with index ${index}`);
    }

    function startContainer(index : number){
        // implémenter la fonction adéquate
        console.log(`Start a container with index ${index}`);
    }

    function stopContainer(index : number){
        //implémenter la fonction adéquate
        console.log(`Stop a container with index ${index}`);
    }


</script>

<div class="w-full">
    {#each containers as container, i}
        <div class="border border-gray-300 rounded p-4 mb-4">
            <h3 class="text-lg font-semibold mb-2">Containers Information</h3>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">ID:</span>
                <span>{container.id}S</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Name:</span>
                <span>{container.name}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Image ID :</span>
                <span>{container.image}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Status :</span>
                <span>{container.status}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Date of creation :</span>
                <span>{container.network}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Ports used :</span>
                <span>{container.ports}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">Ports used :</span>
                <span>{container.volume}</span>
            </div>
            <div class="flex justify-end items-center">
                <a href="/containers/{container.id}">
                <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2">
                   Info
                </button>
                        </a>
                <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => deleteContainer(i)}>
                    Delete
                </button>
                <button class="bg-green-500 text-white px-4 py-2 rounded mr-2" on:click={() => downloadContainer(i)}>
                    Download
                </button>
                <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => stopContainer(i)}>
                    Stop
                </button>
                <button class="bg-blue-500 text-white px-4 py-2 rounded" on:click={() => startContainer(i)}>
                    Start
                </button>
            </div>
        </div>
    {/each}
</div>
