import { TopologyApp } from "./TopologyApp";

export type TopologyInitData = any

export function initApp(canvas: HTMLCanvasElement, parent: HTMLElement, data: TopologyInitData): TopologyApp {
    return new TopologyApp(canvas, parent, data);
}
