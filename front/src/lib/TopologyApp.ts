import * as PIXI from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { TopologyContainer } from './TopologyContainer';

function createBunnyGrid(app: PIXI.Application, x: number, y: number) {
// Create a new texture
const texture = PIXI.Texture.from('https://pixijs.com/assets/bunny.png');

    const container = new PIXI.Container();

// Create a 5x5 grid of bunnies
for (let i = 0; i < 25; i++)
{
    const bunny = new PIXI.Sprite(texture);

    bunny.anchor.set(0.5);
    bunny.x = (i % 5) * 40;
    bunny.y = Math.floor(i / 5) * 40;
    container.addChild(bunny);
}

// Move container to the center
container.x = x;
container.y = y;

// Center bunny sprite in local container coordinates
container.pivot.x = container.width / 2;
container.pivot.y = container.height / 2;

// Listen for animate update
app.ticker.add((delta) =>
{
    // rotate the container!
    // use delta to create frame-independent transform
    container.rotation -= 0.01 * delta;
});

return container;
}

export class TopologyApp {
    app: PIXI.Application;
    viewport: Viewport;

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
}
