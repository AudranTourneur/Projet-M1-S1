import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import { TopologyEntityPixi } from './TopologyEntityPixi';
import type { PortData } from './types/PortData';

export class TopologyPortPixi extends TopologyEntityPixi {
    constructor(public app: TopologyApp, public x: number, public y: number, public data: PortData){
        super(app);
        this.create();
    }

    create(){
        const app = this.app;
		const data = this.data.ip + ':' + this.data.publicPort;
		const x = this.x;
		const y = this.y;

		const container = this.pixiContainer;
		// Create a gray rectangle
		const graphics = new PIXI.Graphics();
		graphics.beginFill(0x00aa00);
		const size = BackgroundGrid.GRID_SIZE;
		graphics.drawRoundedRect(0, 0, size, size, 5);
		graphics.endFill();
		graphics.rotation = Math.PI / 2;
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