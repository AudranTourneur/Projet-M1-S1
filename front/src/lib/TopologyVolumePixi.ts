import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { TopologyEntityPixi } from './TopologyEntityPixi';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyVolume } from './types/TopologyVolume';

export class TopologyVolumePixi extends TopologyEntityPixi {
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

        const SIZE = BackgroundGrid.GRID_SIZE * 0.8;

        graphics.beginFill(colorBg);
        graphics.drawCircle(SIZE, SIZE, SIZE * 2);
        graphics.endFill();
        container.addChild(graphics);

        const dbIconUrl = '/static/database.svg'
        const dbTexture = PIXI.Texture.from(dbIconUrl);
        const dbIcon = new PIXI.Sprite(dbTexture);
        dbIcon.tint = 0x121212;
        dbIcon.width = SIZE * 1.5;
        dbIcon.height = SIZE * 1.5;
        dbIcon.x = SIZE - dbIcon.width / 2;
        dbIcon.y = SIZE - dbIcon.height / 2;
        container.addChild(dbIcon);

        // add text
        const styleName = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 30,
            fill: '#dddddd'
        });

        const idText = new PIXI.Text('Volume', styleName);
        idText.x = 0;
        idText.y = 0;
        // container.addChild(idText);


        const styleImage = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 20,
            fill: '#cccccc'
        });

        // const actualName = randomNamesList[Math.floor(Math.random() * randomNamesList.length)];
        const actualName = data.data.name.substring(0, 20) + '...';

        const text = new PIXI.Text(actualName, styleImage);
        text.x = 0;
        text.y = 3.2 * SIZE;
        container.addChild(text);

        container.x = x;
        container.y = y;
        this.app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(this.app, this);
    }
}