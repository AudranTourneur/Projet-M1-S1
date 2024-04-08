// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ContainerData } from "./ContainerData";
import type { Position } from "./Position";

export type TopologyContainer = { data: ContainerData, name: string, image: string, iconUrl: string, exposedPorts: Array<number>, exposedVolumes: Array<string>, connectedTo: Array<string>, position: Position | null, };