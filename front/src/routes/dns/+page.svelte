<script lang="ts">
    export let data;

    const dns = data.dns;
</script>

<div>DNS</div>

{#each dns as record}
    {record.domainName
    {record.reverseProxyPort}
{/each}