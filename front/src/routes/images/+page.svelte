<script>
    import item from '../info.json';

    function deleteVolume(index) {
        // Implement the logic to delete the volume with the given index
        console.log(`Deleting volume with index ${index}`);
    }
</script>


<h1 class="text-center text-4xl">Images</h1>
<ul>

    {#each item.docker_info.images as {id, name_repository, version, date_images, created_images, size_images}, i}
        <div class="space-y-5">
            <div class="relative p-3 m-2 bg-gray-800 shadow rounded-lg">
                <div>
                    <span>ID : {id}</span>
                    <span class="absolute right-3">{version}</span>
                </div>
                <br/>
                <div>
                    Origine : {name_repository}
                </div>
                <br/>
                <div>
                    <span>Date de création : {date_images}</span>
                    <span class="absolute right-3">{created_images}</span>
                </div>
                <br/>
                <div>
                    taille : {size_images}
                </div>

                <div class="flex justify-end">
                    <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => deleteVolume(i)}>supprimer</button>
                </div>
            </div>
        </div>
    {/each}
</ul>
