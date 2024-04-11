<script lang="ts">
	import type { VolumeData } from '$lib/types/VolumeData';
	import type { PortData } from '$lib/types/PortData';
	import { Fa } from 'svelte-fa';
	import { faPenToSquare, faCheck, faXmark } from '@fortawesome/free-solid-svg-icons';

	export let volume: VolumeData;

    let actualNameValue = volume.name;

	let currentlyEditingValueName: string | null = null;
	// let currentlyEditingValueHostMountpoint: string | null = null;

	function toggleNameEdition() {
        if (currentlyEditingValueName == null) {
            currentlyEditingValueName = actualNameValue;
        } else {
            currentlyEditingValueName = null
        }
	}

    async function confirmNameChange() {
        if (currentlyEditingValueName == null) return;

        actualNameValue = currentlyEditingValueName;
        currentlyEditingValueName = null;
		//A fetch ici
		//const response = await fetch()

    }

    function cancelNameChange() {
        currentlyEditingValueName = null;
    }

    function saveToServer() {
        // await fetch('...')
        // using actualNameValue
    }
</script>

<div>
    Change name

	{#if currentlyEditingValueName == null}
    Current volume name: <span class="font-bold">{actualNameValue}</span>

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
