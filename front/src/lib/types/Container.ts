// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { OurPort } from "./OurPort";

export type Container = { iconUrl: string | null, id: string, names: Array<string>, image: string, networks: Array<string>, volumes: Array<string>, status: string, ports: Array<OurPort>, labels: { [key: string]: string } | null, composeFile: string | null, rawData: string | null, };