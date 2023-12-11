import { TopologyApp } from "./TopologyApp";

export function initApp(canvas: HTMLCanvasElement, parent: HTMLElement): TopologyApp {
    return new TopologyApp(canvas, parent);
}
