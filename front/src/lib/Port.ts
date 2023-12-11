import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';

export class Port {
    constructor(app: TopologyApp, x: number, y: number) {
        // Draw purple losange
        const graphics = new PIXI.Graphics();
        graphics.beginFill(0x800080);
        const size = 50;
        graphics.drawPolygon([
            0, 0,
            size, size,
            0, size * 2,
            -size, size,
        ]);
        graphics.endFill();
        graphics.x = x;
        graphics.y = y;

        const portText = new PIXI.Text('80', {
            fontFamily: 'Arial',
            fontSize: 40,
            fill: '#ffffff'
        });

        portText.x = -size / 2;
        portText.y = size / 2 - 10;

        graphics.addChild(portText);

        app.viewport.addChild(graphics);
    }
}
