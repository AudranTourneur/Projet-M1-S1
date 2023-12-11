import type { TopologyApp } from "./TopologyApp"
import * as PIXI from 'pixi.js';

export class Cable {
    constructor(app: TopologyApp, positions: Array<{x: number, y: number}>) {
        const container = new PIXI.Container();

        const graphics = new PIXI.Graphics();
        graphics.lineStyle(10, 0xcc0000, 1);

        const firstPos = positions.shift();
        if (firstPos) {
            graphics.moveTo(firstPos.x, firstPos.y);
        }


        for (const pos of positions) {
            graphics.lineTo(pos.x, pos.y);
        }
        container.addChild(graphics);

        app.viewport.addChild(container);
    }
}
