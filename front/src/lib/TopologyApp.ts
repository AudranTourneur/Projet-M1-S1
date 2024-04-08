import * as PIXI from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { TopologyContainerPixi } from './TopologyContainerPixi';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyInitData } from './topology';
import { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyVolumePixi } from './TopologyVolume';
import type { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyApp {
	app: PIXI.Application;
	viewport: Viewport;

	currentlySelected: TopologyEntityPixi | null = null;

	allContainers: Array<TopologyContainerPixi> = [];
	allVolumes: Array<TopologyVolumePixi> = [];
	allLinks: Array<TopologyLinkPixi> = [];

	constructor(canvas: HTMLCanvasElement, parent: HTMLElement, public data: TopologyInitData) {
		const app = new PIXI.Application({ background: '#2A547E', resizeTo: parent, view: canvas, antialias: true });

		this.app = app;

		const viewport = new Viewport({
			screenWidth: window.innerWidth,
			screenHeight: window.innerHeight,
			worldWidth: 1000,
			worldHeight: 1000,
			events: app.renderer.events // the interaction module is important for wheel to work properly when renderer.view is placed or scaled
		});

		this.viewport = viewport;

		app.stage.addChild(viewport);

		// activate plugins
		viewport.drag().pinch().wheel().decelerate();

		new BackgroundGrid(this);

		function getRandomCoord() {
			return Math.random() * 2000 - 500;
		}

		for (let i = 0; i < data.containers.length; i++) {
			const container = data.containers[i];
			console.log('set', container)

			const x = container.position?.x ?? getRandomCoord();
			const y = container.position?.y ?? getRandomCoord();
			this.allContainers.push(new TopologyContainerPixi(this, x, y, container));
		}

		for (const source of this.allContainers) {
			for (const targetId of source.data.connectedTo) {
				const target = this.allContainers.find(container => container.data.data.id === targetId);
				if (!target) continue
				TopologyLinkPixi.createLinkIfNeeded(this, source, target)
			}
		}

		for (const volume of data.volumes) {
			const x = volume.position?.x ?? getRandomCoord();
			const y = volume.position?.y ?? getRandomCoord();
			this.allVolumes.push(new TopologyVolumePixi(this, x, y, volume))
		}

		for (const container of this.allContainers) {
			const volumeIds = container.data.data.volumes || []
			const volumes = this.allVolumes.filter(volume => volumeIds.includes(volume.data.data.name))
			console.log('vol', volumeIds, volumes)
			for (const volume of volumes) {
				TopologyLinkPixi.createLinkIfNeeded(this, container, volume)
			}
		}
	}

	select(container: TopologyEntityPixi) {
		this.currentlySelected = container;
		// disable viewport plugins
		this.viewport.plugins.pause('drag');
	}

	unselect() {
		this.currentlySelected = null;
		// enable viewport plugins
		this.viewport.plugins.resume('drag');
	}

	getSaveData(): SaveData {
		return {
			containers: this.allContainers.map(container => ({
				id: container.data.name,
				x: Math.round(container.pixiContainer.x),
				y: Math.round(container.pixiContainer.y),
			}))
		};
	}
}

type SaveData = {
	containers: Array<{
		id: string,
		x: number,
		y: number,
	}>
}