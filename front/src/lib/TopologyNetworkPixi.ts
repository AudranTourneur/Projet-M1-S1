import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import { BackgroundGrid } from './BackgroundGrid';
import type { TopologyContainer } from './types/TopologyContainer';
import type { TopologyLinkPixi } from './TopologyLinkPixi';
import { TopologyEntityPixi } from './TopologyEntityPixi';

export class TopologyNetworkPixi extends TopologyEntityPixi {
    constructor(public app: TopologyApp, public x: number, public y: number, public name: string){
        super(app);
        this.create();
    }

    create(){
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
		idText.x = 0;
		idText.y = 1.6 * size;
		container.addChild(idText);

        container.x = x;
		container.y = y;

		app.viewport.addChild(container);

		TopologyEntityPixi.addDragBehaviour(app, this);
    }

	select() {}

	unselect() {}
}