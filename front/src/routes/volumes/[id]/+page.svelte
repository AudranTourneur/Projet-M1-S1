<script lang="ts">
    export let data;
    const id = data.name;
    console.log(data);





    import {onMount, createEventDispatcher} from "svelte";
	import FileExplorer from "../FileExplorer.svelte";
	import { goto } from '$app/navigation';

    export let data: any;

    let volume: VolumeData = {
        path: '/path/to/volume',
        mountPath: '/mount/path'
    };
    
    let showModal = false;
    const dispatch = createEventDispatcher();


    onMount(async () => {
        const ApexCharts = await import('apexcharts')
        const chart = new ApexCharts.default(document.querySelector("#timeline-chart"), options);
        await chart.render();
    })

    async function handleDeleteVolume(volumeName: String) {
        showModal = true;
      }

    async function confirmDelete(){
        const response = await fetch(`/volumes/${id}/api/remove-volume`, {
             method: 'POST',
        });
        console.log(await response.text())
        showModal = false;
        dispatch('volumeDeleted', { name: data.name});
        goto("/volumes");
    }

    function cancelDelete() {
        showModal = false;
    }

    const options = {
        chart: {
            type: "area",
            height: 300,
            foreColor: "#999",
            stacked: true,
            dropShadow: {
                enabled: true,
                enabledSeries: [0],
                top: -2,
                left: 2,
                blur: 5,
                opacity: 0.06
            }
        },
        colors: ['#00E396', '#0090FF'],
        stroke: {
            curve: "smooth",
            width: 3
        },
        dataLabels: {
            enabled: false
        },
        series: [{
            name: 'Total Views',
            data: generateDayWiseTimeSeries(0, 18)
        }, {
            name: 'Unique Views',
            data: generateDayWiseTimeSeries(1, 18)
        }],
        markers: {
            size: 0,
            strokeColor: "#fff",
            strokeWidth: 3,
            strokeOpacity: 1,
            fillOpacity: 1,
            hover: {
                size: 6
            }
        },
        xaxis: {
            type: "datetime",
            axisBorder: {
                show: false
            },
            axisTicks: {
                show: false
            }
        },
        yaxis: {
            labels: {
                offsetX: 14,
                offsetY: -5
            },
            tooltip: {
                enabled: true
            }
        },
        grid: {
            padding: {
                left: -5,
                right: 5
            }
        },
        tooltip: {
            x: {
                format: "dd MMM yyyy"
            },
        },
        legend: {
            position: 'top',
            horizontalAlign: 'left'
        },
        fill: {
            type: "solid",
            fillOpacity: 0.7
        }
    };


    function generateDayWiseTimeSeries(s: number, count: number) {
        let values = [[
            4,3,10,9,29,19,25,9,12,7,19,5,13,9,17,2,7,5
        ], [
            2,3,8,7,22,16,23,7,11,5,12,5,10,4,15,2,6,2
        ]];
        let i = 0;
        let series = [];
        let x = new Date("11 Nov 2012").getTime();
        while (i < count) {
            series.push([x, values[s][i]]);
            x += 86400000;
            i++;
        }
        return series;
    }

</script>





<div id="chart" class="max-w-760px mx-auto my-8 opacity-90">
    <div id="timeline-chart" class="apexcharts-toolbar-opacity-1 apexcharts-toolbar-border-0"></div>
</div>


<div>
    <br/>
    {data.name}
    <br/>
    {data.createdAt}
    <br/>
    {data.mountpoint}
    <br/>
    {data.size}
    <br/>
    <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={() => handleDeleteVolume(data.name)}>
        Delete
    </button>

    {#if showModal}
    <div class="fixed top-0 left-0 w-full h-full flex items-center justify-center bg-gray-800 bg-opacity-50">
        <div class="bg-black p-4 rounded shadow-md">
            <p>Are you sure you want to delete this volume?</p>
            <div class="flex justify-between mt-4">
                <button class="bg-red-500 text-white px-4 py-2 rounded mr-2" on:click={confirmDelete}>Yes</button>
                <button class="bg-gray-500 text-white px-4 py-2 rounded" on:click={cancelDelete}>No</button>
            </div>
        </div>
    </div>
    {/if}
</div>

<br/>
<div>
    <h2>Filesystem</h2>
    <FileExplorer {id}>
    </FileExplorer>
</div>


<div>
    <br>
    <PortsBox {Volume} />
</div>