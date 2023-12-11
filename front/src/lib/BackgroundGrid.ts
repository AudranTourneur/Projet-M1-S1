import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';

export class BackgroundGrid {
    static readonly GRID_SIZE = 100;

	constructor(app: TopologyApp) {
        const size = 100;
        const lines = 100
        // const maxSize = size * lines;
        const maxSize = (size * lines) / 2;

        const lineWidth = 10;

		for (let i = 0; i < lines; i++) {
			const vertical = new PIXI.Graphics();
            vertical.lineStyle(lineWidth, 0x252e35, 0.5);
			vertical.moveTo(i * size - maxSize, -maxSize);
			vertical.lineTo(i * size - maxSize, maxSize);
			app.viewport.addChild(vertical);
		}

		for (let i = 0; i < lines; i++) {
			const horizontal = new PIXI.Graphics();
			horizontal.lineStyle(lineWidth, 0x252e35, 0.5);
			horizontal.moveTo(-maxSize, i * size - maxSize);
			horizontal.lineTo(maxSize, i * size - maxSize);
			app.viewport.addChild(horizontal);
		}
	}
}
