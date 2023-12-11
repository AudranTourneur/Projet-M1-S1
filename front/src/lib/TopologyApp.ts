import * as PIXI from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { TopologyContainer } from './TopologyContainer';
// import { GraphPaper, GraphStyle } from "pixi-graphpaper";

export class TopologyApp {
    app: PIXI.Application;
    viewport: Viewport;

    currentlySelected: TopologyContainer | null = null;

	constructor(canvas: HTMLCanvasElement, parent: HTMLElement) {
		const app = new PIXI.Application({ background: '#1099bb', resizeTo: parent, view: canvas });

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

// Background grid for reference
// const paper = new GraphPaper(GraphStyle.BLUEPRINT);
// viewport.addChild(paper);

        const coords = [
            [0, 0],
            [500, 0],
            [0, 500],
            [500, 500],
            [250, 250]
        ]

        for (const coor of coords) {
            new TopologyContainer(this, coor[0], coor[1]);
        }
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
