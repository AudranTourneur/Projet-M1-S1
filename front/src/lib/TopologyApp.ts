import * as PIXI from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { TopologyContainer } from './TopologyContainer';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyInitData } from './topology';

export class TopologyApp {
	app: PIXI.Application;
	viewport: Viewport;

	currentlySelected: TopologyContainer | null = null;

	allContainers: Array<TopologyContainer> = [];

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
			this.allContainers.push(new TopologyContainer(this, x, y, container));
			
		}

        // const cable: Array<{x: number, y: number}> = [
        //     {x: 50, y: 0},
        //     {x: 50, y: 250},
        //     {x: 550, y: 250},
        //     {x: 550, y: 500},
        // ];
        //
        // new Cable(this, cable);
        //
        // new Port(this, 10500, 500);
	}

	select(container: TopologyContainer) {
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