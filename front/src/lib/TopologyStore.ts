import { writable, type Writable } from "svelte/store";
import type { TopologyEntityPixi } from "./TopologyEntityPixi";

type SelectedEntity = {entity:TopologyEntityPixi}

export const currentlySelectedEntity:Writable<SelectedEntity | null> = writable(null);