import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';

export class TopologyContainer {
    isDragging: boolean = false;
    

	constructor(app: TopologyApp, x: number, y: number) {
		const container = new PIXI.Container();
		// Create a gray rectangle
		const graphics = new PIXI.Graphics();
		graphics.beginFill(0x808080)
		graphics.drawRoundedRect(0, 0, BackgroundGrid.GRID_SIZE * 3, BackgroundGrid.GRID_SIZE * 2, 20);
		graphics.endFill();
		container.addChild(graphics);

		// add text
		const styleName = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 30,
			fill: '#dddddd'
		});

		const text = new PIXI.Text('my-super-app-1', styleName);
		text.x = 30;
		text.y = 30;
		container.addChild(text);

        const styleImage = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 20,
            fill: '#cccccc'
        });

        const image = new PIXI.Text('ubuntu:22.04', styleImage);
        image.x = 30;
        image.y = 80;
        container.addChild(image);

        const onlineGreenCircle = new PIXI.Graphics();
        onlineGreenCircle.beginFill(0x90EE90);
        onlineGreenCircle.drawCircle(0, 0, 10);
        onlineGreenCircle.endFill();
        onlineGreenCircle.x = 260;
        onlineGreenCircle.y = 170;
        container.addChild(onlineGreenCircle);

        const styleStatus = new PIXI.TextStyle({
            fontFamily: 'Arial',
            fontSize: 20,
            fill: '#90EE90'
        });

        const status = new PIXI.Text('ONLINE', styleStatus);
        status.x = 170;
        status.y = 157;
        container.addChild(status);

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
            const rawX =  pos.x - container.width / 2;
            const rawY = pos.y - container.height / 2
            const gridStep = BackgroundGrid.GRID_SIZE;
            const x = Math.round(rawX / gridStep) * gridStep
            const y = Math.round(rawY / gridStep) * gridStep
            container.x = x;
            container.y = y;
        }

        container.parent.on('pointermove', event => onDrag(event));
	}
}