import * as PIXI from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { TopologyContainerPixi } from './TopologyContainerPixi';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyInitData } from './topology';
import { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyVolumePixi } from './TopologyVolumePixi';
import type { TopologyEntityPixi } from './TopologyEntityPixi';
import { TopologyNetworkPixi } from './TopologyNetworkPixi';
import { TopologyPortPixi } from './TopologyPortPixi';
import { currentlySelectedEntity } from './TopologyStore';
import { simulatePositions } from './GraphSimulation';
import { get } from 'svelte/store';
// import Stats from 'stats.js';


export class TopologyApp {
	app: PIXI.Application;
	viewport: Viewport;

	currentlySelected: TopologyEntityPixi | null = null;

	allContainers: Array<TopologyContainerPixi> = [];
	allVolumes: Array<TopologyVolumePixi> = [];
	allLinks: Array<TopologyLinkPixi> = [];
	allNetworks: Array<TopologyNetworkPixi> = []
	allPorts: Array<TopologyPortPixi> = []

	constructor(canvas: HTMLCanvasElement, parent: HTMLElement, public data: TopologyInitData) {
		const app = new PIXI.Application(
			{
				// backgroundColor: 0x00478F,
				backgroundColor: 0x003D7A,
				resizeTo: parent,
				view: canvas,
				antialias: true
			});

		this.app = app;

		const viewport = new Viewport({
			screenWidth: window.innerWidth,
			screenHeight: window.innerHeight,
			worldWidth: 1000,
			worldHeight: 1000,
			// interaction: app.renderer.interaction,
			interaction: app.renderer.plugins.interaction,
		});

		this.viewport = viewport;

		app.stage.addChild(viewport);

		// activate plugins
		viewport.drag().pinch().wheel().decelerate();

		viewport.sortableChildren = true;

		new BackgroundGrid(this);

		function getRandomCoord() {
			return Math.random() * 2000 - 500;
		}

		for (let i = 0; i < data.containers.length; i++) {
			const container = data.containers[i];
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
			for (const volume of volumes) {
				TopologyLinkPixi.createLinkIfNeeded(this, container, volume)
			}
		}

		let networksSet = new Set<string>()

		for (const container of data.containers) {
			for (const networkName of container.data.networks) {
				networksSet.add(networkName)

			}
		}

		const networksArray = [...networksSet]

		for (const networkName of networksArray) {
			const x = getRandomCoord();
			const y = getRandomCoord();
			this.allNetworks.push(new TopologyNetworkPixi(this, x, y, networkName))
		}

		for (const container of this.allContainers) {
			for (const networkName of container.data.data.networks) {
				const pixiNetworkObj = this.allNetworks.find(n => n.name === networkName)
				if (!pixiNetworkObj) continue;
				TopologyLinkPixi.createLinkIfNeeded(this, container, pixiNetworkObj)
			}
		}

		for (const container of data.containers) {
			for (const port of container.data.ports) {
				if (!port.ip || !port.publicPort) continue
				const portPixi = new TopologyPortPixi(this, getRandomCoord(), getRandomCoord(), port)
				this.allPorts.push(portPixi)

				const containerPixi = this.allContainers.find(c => container.data.id === c.data.data.id)
				if (!containerPixi) continue
				TopologyLinkPixi.createLinkIfNeeded(this, containerPixi, portPixi)
			}
		}

		const allEntities = [...this.allContainers, ...this.allVolumes, ...this.allNetworks, ...this.allPorts]

		simulatePositions(allEntities)



		// const redSquare = new PIXI.Graphics();
		// redSquare.beginFill(0Xffffff);
		// redSquare.drawRoundedRect(0, 0, 100, 100, 20);
		// redSquare.endFill();

		// this.viewport.addChild(redSquare);

		// redSquare.interactive = true;

		// let isRunning = true

		// setInterval(() => {
		// 	if (isRunning) {
		// 		redSquare.x += 1;
		// 		redSquare.y += 1;
		// 		redSquare.rotation += 0.1;
		// 	}
		// }, 20)

		// redSquare.on('pointerdown', () => {
		// 	console.log('??????')
		// 	isRunning = !isRunning
		// 	redSquare.tint = isRunning ? 0Xffffff : 0Xff0000
		// })

		// const stats = new Stats();
		// stats.showPanel(0); // 0: fps, 1: ms, 2: mb, 3+: custom
		// document.body.appendChild(stats.dom);

		// function animate() {

		// 	stats.begin();

		// 	// monitored code goes here

		// 	stats.end();

		// 	requestAnimationFrame(animate);

		// }

		requestAnimationFrame(animate);
	}

	select(entity: TopologyEntityPixi) {
		const previouslySelectedEntity = get(currentlySelectedEntity)
		if (previouslySelectedEntity) {
			previouslySelectedEntity.entity.unselect();
		}

		this.currentlySelected = entity;

		// disable viewport plugins
		this.viewport.plugins.pause('drag');

		currentlySelectedEntity.set({ entity });

		for (const link of entity.links) {
			link.isSelected = true;
			link.update();
		}
		entity.select();
	}

	unselect() {
		if (!this.currentlySelected) return;
		for (const link of this.currentlySelected.links) {
			link.isSelected = false;
			link.update();
		}
		// this.currentlySelected.unselect();
		// this.currentlySelected.unselect();

		this.currentlySelected = null;
		// enable viewport plugins
		this.viewport.plugins.resume('drag');

	}

	getSaveData(): SaveData {
		return {
			containers: this.allContainers.map(container => ({
				id: container.data.data.id,
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