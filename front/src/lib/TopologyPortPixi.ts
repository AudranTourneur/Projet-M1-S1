import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyPortPixi extends TopologyEntityPixi {
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
		graphics.beginFill(0x00FF00);
		graphics.drawRoundedRect(0, 0, BackgroundGrid.GRID_SIZE * 3, BackgroundGrid.GRID_SIZE * 2, 20);
		graphics.endFill();
		container.addChild(graphics);

        // add text
		const styleName = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 30,
			fill: '#dddddd'
		});

		const idText = new PIXI.Text(data, styleName);
		idText.x = 30;
		idText.y = 30;
		container.addChild(idText);

        container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(app, this);
    }
}