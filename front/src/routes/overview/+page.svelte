<script lang="ts">

    export let data;

    console.log(data);

    let overview = data;

</script>

<div class="w-full">
        <div class="border border-gray-300 rounded p-4 mb-4">
            <h3 class="text-lg text-center font-semibold mb-2">Overview</h3>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">versionDocker:</span>
                <span>{overview.versionDocker}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">versionLinux:</span>
                <span>{overview.versionLinux}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">images:</span>
                <span>{overview.images}</span>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">containers:</span>
                <span>{overview.containers}</span>
            </div>

            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">volumes:</span>
                <span>{overview.volumes}</span>
            </div>

            <div class="flex justify-between items-center mb-2">
                <span class="font-bold">networks:</span>
                <span>{overview.networks}</span>
            </div>
        </div>
</div>
