import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';

export class TopologyContainer {
	isDragging: boolean = false;

	constructor(app: TopologyApp, x: number, y: number, data: any) {
		const container = new PIXI.Container();
		// Create a gray rectangle
		const graphics = new PIXI.Graphics();
		graphics.beginFill(0x303338);
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

		const idText = new PIXI.Text('Container', styleName);
		idText.x = 30;
		idText.y = 30;
		container.addChild(idText);

		const styleImage = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 20,
			fill: '#cccccc'
		});

		// const actualName = randomNamesList[Math.floor(Math.random() * randomNamesList.length)];
		const actualName = data.image;

		const image = new PIXI.Text(actualName, styleImage);
		image.x = 30;
		image.y = 80;
		container.addChild(image);

		const isOnline = true;

		if (isOnline) {
			const onlineGreenCircle = new PIXI.Graphics();
			onlineGreenCircle.beginFill(0x5ba65b);
			onlineGreenCircle.drawCircle(0, 0, 10);
			onlineGreenCircle.endFill();
			onlineGreenCircle.x = 260;
			onlineGreenCircle.y = 170;
			container.addChild(onlineGreenCircle);
			setInterval(() => {
				onlineGreenCircle.visible = !onlineGreenCircle.visible;
			}, 500);

			const styleStatus = new PIXI.TextStyle({
				fontFamily: 'Arial',
				fontSize: 20,
				fill: '#5BA65B'
			});

			const status = new PIXI.Text('ONLINE', styleStatus);
			status.x = 170;
			status.y = 157;
			container.addChild(status);
		} else {
			// #D13A3A

			const offlineRedCircle = new PIXI.Graphics();
			offlineRedCircle.beginFill(0xd13a3a);
			offlineRedCircle.drawCircle(0, 0, 10);
			offlineRedCircle.endFill();
			offlineRedCircle.x = 260;
			offlineRedCircle.y = 170;
			container.addChild(offlineRedCircle);

			const styleStatus = new PIXI.TextStyle({
				fontFamily: 'Arial',
				fontSize: 20,
				fill: '#D13A3A'
			});

			const status = new PIXI.Text('OFFLINE', styleStatus);
			status.x = 160;
			status.y = 157;
			container.addChild(status);
		}

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
		});

		const onDrag = (event: PIXI.FederatedPointerEvent) => {
			if (!this.isDragging) return;
			const pos = event.data.getLocalPosition(app.viewport);
			const rawX = pos.x - container.width / 2;
			const rawY = pos.y - container.height / 2;
			const gridStep = BackgroundGrid.GRID_SIZE;
			const x = Math.round(rawX / gridStep) * gridStep;
			const y = Math.round(rawY / gridStep) * gridStep;
			container.x = x;
			container.y = y;
		};

		container.parent.on('pointermove', (event) => onDrag(event));
	}
}
