import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyContainer } from './types/TopologyContainer';
import type { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyNetworkPixi extends TopologyEntityPixi {

	private selectedRouter: PIXI.Sprite | null = null;

	constructor(public app: TopologyApp, public x: number, public y: number, public name: string) {
		super(app);
		this.create();
	}

	create() {
		const app = this.app;
		const data = this.name;
		const x = this.x;
		const y = this.y;

		const container = this.pixiContainer;
		const size = BackgroundGrid.GRID_SIZE * 1.5;

		const routerIconUrl = '/static/router.svg'
		const routerTexture = PIXI.Texture.from(routerIconUrl);
		const routerIcon = new PIXI.Sprite(routerTexture);

		routerIcon.width = size * 1.5;
		routerIcon.height = size * 1.5;

		container.addChild(routerIcon);

		// add text
		const styleName = new PIXI.TextStyle({
			fontFamily: 'Arial',
			fontSize: 40,
			fill: '#dddddd'
		});

		const idText = new PIXI.Text(data, styleName);
		const idTextWidth = PIXI.TextMetrics.measureText(data, styleName).width;
	
		idText.x = size - idTextWidth/1.5;
		idText.y = 1.6 * size;
		container.addChild(idText);

		container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		
		/* const orange = 0xffa500;
		const selectedRouterUrl = '/static/router.svg'
		const selectedRouterTexture = PIXI.Texture.from(selectedRouterUrl);
		const selectedRouterSprite = new PIXI.Sprite(selectedRouterTexture);
		selectedRouterSprite.tint = orange;
		selectedRouterSprite.visible = false;
		container.addChild(selectedRouterSprite); */

		TopologyEntityPixi.addDragBehaviour(app, this);
	}


	select() {}
	unselect(){}
	/* select(): void {
		if (this.selectedRouter) {
			this.selectedRouter.visible = true;
		}
	}

	unselect(): void {
		if (this.selectedRouter) {
			this.selectedRouter.visible = false;
		}
	} */
}