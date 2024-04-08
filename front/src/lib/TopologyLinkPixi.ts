import * as PIXI from 'pixi.js';
import type { TopologyApp } from "./TopologyApp";
import type { TopologyContainerPixi } from "./TopologyContainer";

export class TopologyLink {
    static createLink(app: TopologyApp, source: TopologyContainerPixi, target: TopologyContainerPixi) {
        console.log('Creating link')
        const link = new PIXI.Graphics();
        link.lineStyle(2, 0x000000, 1);
        link.moveTo(source.pixiContainer.x + source.pixiContainer.width / 2, source.pixiContainer.y + source.pixiContainer.height / 2);
        link.lineTo(target.pixiContainer.x + target.pixiContainer.width / 2, target.pixiContainer.y + target.pixiContainer.height / 2);
        app.viewport.addChild(link);
    }
}