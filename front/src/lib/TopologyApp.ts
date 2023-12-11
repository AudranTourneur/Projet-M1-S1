import * as PIXI from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { TopologyContainer } from './TopologyContainer';
import { BackgroundGrid } from './BackgroundGrid';
import { Cable } from './Cable';
import { Port } from './Port';

export class TopologyApp {
	app: PIXI.Application;
	viewport: Viewport;

	currentlySelected: TopologyContainer | null = null;

	constructor(canvas: HTMLCanvasElement, parent: HTMLElement) {
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

		const coords = [
			[0, -200],
			[500, 0],
			[0, 500],
			[500, 500],
			[900, 300]
		];

		for (const coor of coords) {
			new TopologyContainer(this, coor[0], coor[1]);
		}

        const cable: Array<{x: number, y: number}> = [
            {x: 50, y: 0},
            {x: 50, y: 250},
            {x: 550, y: 250},
            {x: 550, y: 500},
        ];

        new Cable(this, cable);

        new Port(this, 1050, 0);
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
}
