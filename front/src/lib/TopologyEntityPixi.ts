import * as PIXI from 'pixi.js';
import type { TopologyApp } from './TopologyApp';
import type { TopologyLinkPixi } from './TopologyLinkPixi';
import { BackgroundGrid } from './BackgroundGrid';
import { nanoid } from 'nanoid'


export abstract class TopologyEntityPixi {
    isDragging: boolean = false;
    pixiContainer = new PIXI.Container();
    links: Array<TopologyLinkPixi> = [];
	uuid: string = nanoid();

    constructor(public app: TopologyApp) {
        // this.create()
    }

    abstract create(): void;

    static addDragBehaviour(app: TopologyApp, entity: TopologyEntityPixi) {
		const container = entity.pixiContainer;

		container.interactive = true;

		container.on('pointerdown', () => {
			container.alpha = 0.5;
			entity.isDragging = true;
			app.select(entity);
		});

		container.on('pointerup', () => {
			container.alpha = 1;
			entity.isDragging = false;
			app.unselect();
		});

		container.on('pointerupoutside', () => {
			container.alpha = 1;
			entity.isDragging = false;
			app.unselect();
		});

		const onDrag = (event: PIXI.FederatedPointerEvent) => {
			if (!entity.isDragging) return;
			const pos = event.data.getLocalPosition(app.viewport);
			const rawX = pos.x - container.width / 2;
			const rawY = pos.y - container.height / 2;
			const gridStep = BackgroundGrid.GRID_SIZE;
			const x = Math.round(rawX / gridStep) * gridStep;
			const y = Math.round(rawY / gridStep) * gridStep;
			// const width = container.width;
			// const height = container.height;
			container.x = x;
			container.y = y;

			for (const link of entity.links) {
				link.update()
			}
		};

		container.parent.on('pointermove', (event) => onDrag(event));
    }
}

