import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import type { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyEntityPixi } from './TopologyEntityPixi';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyVolume } from './types/TopologyVolume';

export class TopologyVolumePixi extends TopologyEntityPixi {
    isDragging: boolean = false;
    pixiContainer = new PIXI.Container();
    links: Array<TopologyLinkPixi> = [];

    constructor(public app: TopologyApp, public x: number, public y: number, public data: TopologyVolume) {
        super(app);
        this.create();
    }

    create(): void {
        const data = this.data;
        const x = this.x;
        const y = this.y;

        const container = this.pixiContainer;
        // Create a gray rectangle
        const graphics = new PIXI.Graphics();


        // a gray with a bit of green
        const colorBg = 0x4d6b53;

        graphics.beginFill(colorBg);
        graphics.drawRoundedRect(0, 0, BackgroundGrid.GRID_SIZE * 3, BackgroundGrid.GRID_SIZE * 2, 20);
        graphics.endFill();
        container.addChild(graphics);

        // add text
        const styleName = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 30,
            fill: '#dddddd'
        });

        const nb = Math.floor(Math.random() * 100);

        const idText = new PIXI.Text('Volume', styleName);
        idText.x = 30;
        idText.y = 30;
        container.addChild(idText);


        const styleImage = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 20,
            fill: '#cccccc'
        });

        // const actualName = randomNamesList[Math.floor(Math.random() * randomNamesList.length)];
        const actualName = data.data.name.substring(0, 20) + '...';

        const image = new PIXI.Text(actualName, styleImage);
        image.x = 30;
        image.y = 80;
        container.addChild(image);

        container.x = x;
        container.y = y;
        this.app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(this.app, this);
    }
}