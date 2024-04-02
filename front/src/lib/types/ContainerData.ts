// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { PortData } from "./PortData";

export type ContainerData = { iconUrl: string | null, id: string, names: Array<string>, image: string, networks: Array<string>, volumes: Array<string>, status: string, ports: Array<PortData>, labels: { [key: string]: string } | null, composeFile: string | null, rawData: string | null, isRunning: boolean, };