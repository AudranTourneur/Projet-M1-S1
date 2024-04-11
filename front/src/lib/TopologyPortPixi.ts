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
		// purple
		graphics.beginFill(0xa44cd3);
		const size = BackgroundGrid.GRID_SIZE * 0.8;
		graphics.drawRoundedRect(0, 0, size, size, 5);
		graphics.endFill();
		graphics.angle = 45;
		container.addChild(graphics);

        // add text
		const styleName = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 30,
			fill: '#dddddd'
		});

		const idText = new PIXI.Text(data, styleName);
		idText.x = 0;
		idText.y = 1.4 * size;
		container.addChild(idText);

        container.x = x;
		container.y = y;

        const portIconUrl = '/static/plug.svg'
        const portTexture = PIXI.Texture.from(portIconUrl);
        const portIcon = new PIXI.Sprite(portTexture);

		const scaleIcon = 0.7;

		portIcon.width = size * scaleIcon;
		portIcon.height = size * scaleIcon;

		portIcon.x = -size * scaleIcon/2;
		portIcon.y = size * scaleIcon/2;

		portIcon.tint = 0x444444;

		container.addChild(portIcon);

		app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(app, this);
    }
}