<script>
    import item from '../info.json';
    let images = item.docker_info.images;

</script>
<ul>
    {#each images as {id, name_repository, version, date_images, created_images, size_images}, i}
        <li>{id}</li>
    {/each}
</ul>
