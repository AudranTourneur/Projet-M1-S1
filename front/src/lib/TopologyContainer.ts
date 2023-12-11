import * as PIXI from "pixi.js";
import type { TopologyApp } from "./TopologyApp";

export class TopologyContainer {
    constructor(app: TopologyApp, x: number, y: number) {
        const container = new PIXI.Container();

        // Create a gray rectangle
        const graphics = new PIXI.Graphics();
        graphics.beginFill(0x808080);
        graphics.drawRect(0, 0, 100, 100);
        graphics.endFill();
        container.addChild(graphics);

        // add text
        const style = new PIXI.TextStyle({
            fontFamily: "Arial",
            fontSize: 12,
            fill: "white",
        });

        const text = new PIXI.Text("Container", style);
        text.x = 10;
        text.y = 10;
        container.addChild(text);

        container.x = x;
        container.y = y;

        app.viewport.addChild(container);
    }
}
