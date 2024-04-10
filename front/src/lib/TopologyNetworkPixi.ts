import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyContainer } from './types/TopologyContainer';
import type { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyNetworkPixi extends TopologyEntityPixi {
    constructor(public app: TopologyApp, public x: number, public y: number, public name: string){
        super(app);
        this.create();
    }

    create(){
        const app = this.app;
		const data = this.name;
		const x = this.x;
		const y = this.y;

		const container = this.pixiContainer;
		// Create a gray rectangle
		const graphics = new PIXI.Graphics();
		graphics.beginFill(0xFF0000);
		graphics.drawRoundedRect(0, 0, BackgroundGrid.GRID_SIZE * 3, BackgroundGrid.GRID_SIZE * 2, 20);
		graphics.endFill();
		container.addChild(graphics); 

        container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(app, this);
    }
}