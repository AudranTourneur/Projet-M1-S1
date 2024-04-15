<script lang="ts">
	import { page } from '$app/stores';
	import { formatBytes } from '$lib/FormatUtils.js';
	import { onMount } from 'svelte';

	export let inputData: Array<[number, number]>;

	onMount(async () => {
		const response = await fetch('/containers/' + $page.params.id + '/api/stats');
		const stats = await response.json();

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
					name: 'Memory used',
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
					offsetX: -20,
					formatter: function (val, index) {
						return formatBytes(val);
					}
					// formatter: function (value, { series, seriesIndex, dataPointIndex, w }) {
					// }
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
			return stats.map((obj) => {
				return [obj.ts * 1000, obj.mem];
			});
		}

		const ApexCharts = await import('apexcharts');
		const chart = new ApexCharts.default(document.querySelector('#timeline-chart'), options);
		await chart.render();
	});
</script>

<div id="chart" class="max-w-760px mx-auto my-8 opacity-90">
	<div id="timeline-chart" class="apexcharts-toolbar-opacity-1 apexcharts-toolbar-border-0"></div>
</div>
