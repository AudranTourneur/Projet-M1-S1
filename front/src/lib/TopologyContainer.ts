import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';

export class TopologyContainer {
    isDragging: boolean = false;
    

	constructor(app: TopologyApp, x: number, y: number) {
		const container = new PIXI.Container();

		// Create a gray rectangle
		const graphics = new PIXI.Graphics();
		graphics.beginFill(0x808080);
		graphics.drawRect(0, 0, 100, 100);
		graphics.endFill();
		container.addChild(graphics);

		// add text
		const style = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 12,
			fill: 'white'
		});

		const text = new PIXI.Text('Container', style);
		text.x = 10;
		text.y = 10;
		container.addChild(text);

		container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		container.interactive = true;

		container.on('pointerdown', () => {
			container.alpha = 0.5;
            this.isDragging = true;
            app.select(this);
		});

        container.on('pointerup', () => {
            container.alpha = 1;
            this.isDragging = false;
            app.unselect();
        });

        container.on('pointerupoutside', () => {
            container.alpha = 1;
            this.isDragging = false;
            app.unselect();
        })


        const onDrag = (event: PIXI.FederatedPointerEvent) => {
            if (!this.isDragging) return;
            const pos = event.data.getLocalPosition(app.viewport);
            container.x = pos.x - container.width / 2;
            container.y = pos.y - container.height / 2;
        }

        container.parent.on('pointermove', event => onDrag(event));
	}
}
