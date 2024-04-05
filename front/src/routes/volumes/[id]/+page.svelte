<script lang="ts">
    export let data;
    const id = data.name;
    console.log(data);

    import {onMount} from "svelte";
	import FileExplorer from "../FileExplorer.svelte";

    onMount(async () => {
        const ApexCharts = await import('apexcharts')
        const chart = new ApexCharts.default(document.querySelector("#timeline-chart"), options);
        await chart.render();
    })

    async function handleDeleteVolume(volumeName: String) {
        const name = volumeName;
        const response = await fetch(`/volumes/${id}/api/remove-volume`, {
             method: 'POST',
        });
        console.log(await response.text())
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
</div>

<br/>
<div>
    <h2>Filesystem</h2>
    <FileExplorer {id}>
    </FileExplorer>
</div>