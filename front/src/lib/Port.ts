import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';

export class Port {
    constructor(app: TopologyApp, x: number, y: number) {
        const container = new PIXI.Container();
        // Draw purple losange
        const graphics = new PIXI.Graphics();
        graphics.beginFill(0x800080);
        const size = 75;
        graphics.drawRoundedRect(0, 0, size, size, 10);
       
        graphics.endFill();
        container.x = x;
        container.y = y;

        // rotate graphics
        graphics.rotation = 45 * Math.PI / 180;

        const portText = new PIXI.Text('80', {
            fontFamily: 'Arial',
            fontSize: 40,
            fill: '#ffffff'
        });

        container.addChild(portText);
        container.addChild(graphics);

        app.viewport.addChild(graphics);
    }
}
