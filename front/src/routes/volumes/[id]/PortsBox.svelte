<script lang="ts">
	import type { VolumeData } from '$lib/types/VolumeData';
	import type { PortData } from '$lib/types/PortData';
	import { Fa } from 'svelte-fa';
	import {onMount, createEventDispatcher} from "svelte";
	import { faPenToSquare, faCheck, faXmark } from '@fortawesome/free-solid-svg-icons';

	export let volume: VolumeData;

	const dispatch = createEventDispatcher();




    let actualNameValue = volume.name;
	const id = actualNameValue;

	let currentlyEditingValueName: string | null = null;
	// let currentlyEditingValueHostMountpoint: string | null = null;

	function toggleNameEdition() {
        if (currentlyEditingValueName == null) {
            currentlyEditingValueName = actualNameValue;
        } else {
            currentlyEditingValueName = null
        }
	}

	function confirmNameChange() {
        if (currentlyEditingValueName == null) return;

        actualNameValue = currentlyEditingValueName;
        currentlyEditingValueName = null;
		saveToServer(actualNameValue)

    }

    function cancelNameChange() {
        currentlyEditingValueName = null;
    }

    async function saveToServer(actualNameValue: any) {
        const response = await fetch(`/volumes/${id}/api/change-name`, {
			method: 'POST',
			headers: {
            'Content-Type': 'application/json'
        },
			body: JSON.stringify({ name: actualNameValue})
		});

		await response.json();

		dispatch('Name changed on server with', {actualNameValue} );
    }
</script>

<div>
	{#if currentlyEditingValueName == null}
	<span class="font-bold">{actualNameValue}</span>

		<button class="btn-icon variant-filled" on:click={toggleNameEdition}>
			<Fa icon={faPenToSquare} />
		</button>
	{:else}
		<input
			title="Input (text)"
			type="text"
			bind:value={currentlyEditingValueName}
			placeholder="new volume name"
			class="input w-[800px]" />

		<button class="btn-icon variant-filled" on:click={confirmNameChange}>
			<Fa icon={faCheck} />
		</button>

		<button class="btn-icon variant-filled" on:click={cancelNameChange}>
			<Fa icon={faXmark} />
		</button>
	{/if}

	<br />
</div>
