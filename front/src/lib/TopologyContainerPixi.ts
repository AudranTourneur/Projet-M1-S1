import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyContainer } from './types/TopologyContainer';
import { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyContainerPixi extends TopologyEntityPixi {
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
			fontSize: 25,
			fill: '#dddddd'
		});

		let actualName;
		if (data.data.image.length > 20) {
			actualName = data.data.image.substring(0, 20) + '...';
		} else {
			actualName = data.data.image;
		}

		const idText = new PIXI.Text(actualName, styleName);
		idText.x = 30;
		idText.y = 30;
		container.addChild(idText);


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

		if (data.data.iconUrl) {
			const img = PIXI.Sprite.from(data.data.iconUrl);
			console.log('IMG', img)
			// const img = PIXI.Sprite.from('https://cdn-icons-png.flaticon.com/512/888/888879.png')
			img.width = BackgroundGrid.GRID_SIZE * 1;
			img.height = BackgroundGrid.GRID_SIZE * 1;

			img.x = 0.2 * BackgroundGrid.GRID_SIZE;
			img.y = 0.9 * BackgroundGrid.GRID_SIZE;

			container.addChild(img);
		}

		container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(app, this);
	}
}
