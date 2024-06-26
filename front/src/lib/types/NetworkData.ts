// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { IpamConfigData } from "./IpamConfigData";
import type { NetworkContainerData } from "./NetworkContainerData";

export type NetworkData = { id: string, name: string, created: string, labels: { [key: string]: string } | null, ipamConfig: Array<IpamConfigData> | null, containers: { [key: string]: NetworkContainerData } | null, };