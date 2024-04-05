<script lang="ts">

import { deleteVolume } from './[id]/filesystem/[path]/api/remove-volume/+server.ts';

    export let data;

    const volumes = data.volumes;
    
    function handleDeleteVolume(index) {
        deleteVolume(index)
            .then(response => {
                console.log(response.message); // Afficher le message de suppression réussie
                // Vous pouvez mettre à jour l'interface utilisateur ou effectuer d'autres actions en fonction de la réponse
            })
            .catch(error => {
                console.error(error); // Gérer les erreurs, par exemple afficher un message d'erreur à l'utilisateur
            });
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
                <a href="/volumes/{volume.name}">
                    <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2">
                        Info
                    </button>
                </a>
                <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => handleDeleteVolume(index)}>
                    Delete
                </button>
                <button class="bg-blue-500 text-white px-4 py-2 rounded" on:click={() => downloadVolume(i)}>
                    Download
                </button>
            </div>
        </div>
    {/each}
</div>
