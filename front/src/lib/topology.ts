import { TopologyApp } from "./TopologyApp";
import type { Topology } from "./types/Topology";

export type TopologyInitData = Topology

export function initApp(canvas: HTMLCanvasElement, parent: HTMLElement, data: TopologyInitData): TopologyApp {
    return new TopologyApp(canvas, parent, data);
}
