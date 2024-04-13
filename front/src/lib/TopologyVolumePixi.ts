import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { TopologyEntityPixi } from './TopologyEntityPixi';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyVolume } from './types/TopologyVolume';
import { selectionColor } from './Constants';

export class TopologyVolumePixi extends TopologyEntityPixi {

    private selectionCircle: PIXI.Graphics | null = null;

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

        const size = BackgroundGrid.GRID_SIZE * 1.3;

        graphics.beginFill(colorBg);
        graphics.drawCircle(size, size, size);
        graphics.endFill();
        container.addChild(graphics);

        this.actualCenter = {
            x: size,
            y: size
        }

        const dbIconUrl = '/static/database.svg'
        const dbTexture = PIXI.Texture.from(dbIconUrl);
        const dbIcon = new PIXI.Sprite(dbTexture);
        dbIcon.tint = 0x121212;
        dbIcon.width = size * 1;
        dbIcon.height = size * 1;
        dbIcon.x = size - dbIcon.width / 2;
        dbIcon.y = size - dbIcon.height / 2;
        container.addChild(dbIcon);

        const styleImageName = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 30,
            fill: '#cccccc'
        });

        
        const actualName = data.data.name;
        const MAX_SIZE = 20;
        const nameToDisplay = actualName.length > MAX_SIZE ? (actualName.substring(0, MAX_SIZE) + '...') : actualName;

        const text = new PIXI.Text(nameToDisplay, styleImageName);

        const textWidth = PIXI.TextMetrics.measureText(nameToDisplay, styleImageName).width;

        text.x = size - textWidth / 2;
        text.y = 2.3 * size;
        container.addChild(text);

        container.x = x;
        container.y = y;
        this.app.viewport.addChild(container);

        const orangeCircle = new PIXI.Graphics();
        orangeCircle.lineStyle(5, selectionColor, 1);
        orangeCircle.drawCircle(size, size, size);
        orangeCircle.endFill();
        orangeCircle.visible = false;
        container.addChild(orangeCircle);

        this.selectionCircle = orangeCircle;

		TopologyEntityPixi.addDragBehaviour(this.app, this);
    }

    select(): void {
        if (this.selectionCircle) {
            this.selectionCircle.visible = true;
        } 
    }

    unselect(): void {
        if (this.selectionCircle) {
            this.selectionCircle.visible = false;
        }
    }
}