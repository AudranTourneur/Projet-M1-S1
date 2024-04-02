<script lang="ts">
import {onMount} from "svelte";
import {Base64} from "js-base64";

export let id : string;
let path = "/";
let files = [];
let directories = [];

onMount(async () => {
    const response = await fetch(`/volumes/${id}/filesystem/${Base64.encodeURI(path)}/api`);
    const res = await response.json();
    console.log(res);
})



</script>

<!-- {#each files as file}
    <div>{file.name}</div>
{/each} -->