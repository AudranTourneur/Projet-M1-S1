<script lang="ts">
	export let data;
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	onMount(async () => {

		const response = await fetch('/containers/' + $page.params.id + '/api/stats');
		const stats = await response.json();
		console.log(stats);

		const options = {
			chart: {
				type: 'area',
				height: 300,
				foreColor: '#999',
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
				curve: 'smooth',
				width: 3
			},
			dataLabels: {
				enabled: false
			},
			series: [
				{
					name: 'mémoire utilisé',
					data: generateDayWiseTimeSeries(stats.stats)
				}
			],
			markers: {
				size: 0,
				strokeColor: '#fff',
				strokeWidth: 3,
				strokeOpacity: 1,
				fillOpacity: 1,
				hover: {
					size: 6
				}
			},
			xaxis: {
				type: 'datetime',
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
					format: 'h tt m'
				}
			},
			legend: {
				position: 'top',
				horizontalAlign: 'left'
			},
			fill: {
				type: 'solid',
				fillOpacity: 0.7
			}
		};

		function generateDayWiseTimeSeries(
			stats: Array<{ ts: number; mem: number; cpu: number }>
		): Array<[number, number]> {
			return stats.map(obj => {
                return [obj.ts * 1000 , obj.mem]
            })
		}

		const ApexCharts = await import('apexcharts');
		const chart = new ApexCharts.default(document.querySelector('#timeline-chart'), options);
		await chart.render();
	});

	
</script>

<div id="chart" class="max-w-760px mx-auto my-8 opacity-90">
	<div id="timeline-chart" class="apexcharts-toolbar-opacity-1 apexcharts-toolbar-border-0"></div>
</div>



<div class="border-token border-surface-300-600-token rounded-container-token p-4 mb-4">
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">ID:</span>
        <span>{data.id}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Name:</span>
        <span>{data.names}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Image ID :</span>
        <span>{data.image}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Status :</span>
        <span>{data.status}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Date of creation :</span>
        <span>{data.network}</span>
    </div>
    <div class="flex justify-between items-center mb-2">
        {#each data.ports as port}
            <span>IP: {port.ip}</span>
            <span>PrivatePort: {port.privatePort}</span>
            <span>PublicPort: {port.publicPort}</span>
            <span>Type: {port.type}</span>
        {/each}
    </div>
    <div class="flex justify-between items-center mb-2">
        <span class="font-bold">Ports used :</span>
        <span>{data.volumes}</span>
    </div>
</div>