<script lang="ts">
    import type {VolumeStatsResponse} from '$lib/types/VolumeStatsResponse';
    import type {VolumeRow} from '$lib/types/VolumeRow';
    import {page} from '$app/stores';
    import {onMount} from 'svelte';
    import LineChartBytes from './LineChartBytes.svelte';

    export let volumeNameBase64 : string;

    let statVolume: null | Array<[number, number]> = null;
    console.log("string sent to volumechart" + volumeNameBase64);
    function generateTimeDsk(stats: VolumeRow[]): Array<[number, number]> {
        return stats.map((obj) => {
            return [Number(obj.ts) * 1000, Number(obj.dsk)];
        });
    }

    onMount(async () => {
        const statsUrl = '/volumes/' + volumeNameBase64 + '/api/stats'
        console.log('iciiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii =>', volumeNameBase64, 'url=', statsUrl)
        const response = await fetch(statsUrl);
        const statsRes = (await response.json()) as VolumeStatsResponse;
        console.log('iciiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii response =>', statsRes)
        statVolume = generateTimeDsk(statsRes.stats);
        console.log(statVolume)
    });
    
    
</script>

{#if statVolume}
    <LineChartBytes inputData={statVolume} secondinputData={null} name1={'Disk usage'} name2={null} isCpu={false}/>
{:else}
    <div>Loading...</div>

{/if}