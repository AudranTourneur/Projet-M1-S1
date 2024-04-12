import * as PIXI from 'pixi.js';
import type { TopologyApp } from "./TopologyApp";
import type { TopologyEntityPixi } from "./TopologyEntityPixi";

const size = 10
const color = 0x222222

export class TopologyLinkPixi {
    constructor(public source: TopologyEntityPixi, public target: TopologyEntityPixi, public link: PIXI.Graphics) {}

    static createLinkIfNeeded(app: TopologyApp, source: TopologyEntityPixi, target: TopologyEntityPixi) {
        console.log('Creating link')
        const link = new PIXI.Graphics();
        link.lineStyle(size, color, 1);
        link.moveTo(source.pixiContainer.x + source.pixiContainer.width / 2, source.pixiContainer.y + source.pixiContainer.height / 2);
        link.lineTo(target.pixiContainer.x + target.pixiContainer.width / 2, target.pixiContainer.y + target.pixiContainer.height / 2);
        link.zIndex = -1;

        app.viewport.addChild(link);

        const linkClass = new TopologyLinkPixi(source, target, link);

        source.links.push(linkClass);
        target.links.push(linkClass);
    }

    update() {
        this.link.clear();
        this.link.lineStyle(size, color, 1);
        this.link.moveTo(this.source.pixiContainer.x + this.source.pixiContainer.width / 2, this.source.pixiContainer.y + this.source.pixiContainer.height / 2);
        this.link.lineTo(this.target.pixiContainer.x + this.target.pixiContainer.width / 2, this.target.pixiContainer.y + this.target.pixiContainer.height / 2);
    }
}