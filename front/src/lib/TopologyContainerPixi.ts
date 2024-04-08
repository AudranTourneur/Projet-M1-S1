import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyContainer } from './types/TopologyContainer';
import type { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyContainerPixi extends TopologyEntityPixi {
	isDragging: boolean = false;
	pixiContainer = new PIXI.Container();
	links: Array<TopologyLinkPixi> = [];

	constructor(public app: TopologyApp, public x: number, public y: number, public data: TopologyContainer) {
		super(app);
		this.create();
	}

	create() {
		const app = this.app;
		const data = this.data;
		const x = this.x;
		const y = this.y;

		const container = this.pixiContainer;
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
		const actualName = data.data.image;

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

		// if (data.iconUrl) {
			// const img = PIXI.Sprite.from(data.iconUrl);
			const img = PIXI.Sprite.from('https://cdn-icons-png.flaticon.com/512/888/888879.png')
			img.width = BackgroundGrid.GRID_SIZE * 0.4;
			img.height = BackgroundGrid.GRID_SIZE * 0.4;

			img.x = 0.2 * BackgroundGrid.GRID_SIZE;
			img.y = 1.4 * BackgroundGrid.GRID_SIZE;

			container.addChild(img);
		// }

		container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(app, this);
	}
}
