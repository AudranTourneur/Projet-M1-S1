import * as PIXI from 'pixi.js';
import type { TopologyApp } from "./TopologyApp";
import type { TopologyEntityPixi } from "./TopologyEntityPixi";
import { selectionColor } from './Constants';

const size = 10
const color = 0x222222

export class TopologyLinkPixi {
    isSelected: boolean = false;

    constructor(public source: TopologyEntityPixi, public target: TopologyEntityPixi, public link: PIXI.Graphics) {}

    static createLinkIfNeeded(app: TopologyApp, source: TopologyEntityPixi, target: TopologyEntityPixi) {
        const link = new PIXI.Graphics();
        link.zIndex = -1;

        app.viewport.addChild(link);

        const linkClass = new TopologyLinkPixi(source, target, link);

        source.links.push(linkClass);
        target.links.push(linkClass);
        
        linkClass.update();
    }

    update() {
        this.link.clear();
        this.link.lineStyle(size, !this.isSelected ? color : selectionColor, 1);
        this.link.moveTo(this.source.pixiContainer.x + this.source.actualCenter.x, this.source.pixiContainer.y + this.source.actualCenter.y);
        this.link.lineTo(this.target.pixiContainer.x + this.target.actualCenter.x, this.target.pixiContainer.y + this.target.actualCenter.y);
    }
}