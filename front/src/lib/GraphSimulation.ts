import Graph from 'graphology';
import forceLayout from 'graphology-layout-force';
import circular from 'graphology-layout/circular';

import type { TopologyEntityPixi } from './TopologyEntityPixi';

export async function simulatePositions(entities: TopologyEntityPixi[]) {
    const graph = new Graph();
    for (const entity of entities) {
        graph.addNode(entity.uuid);
    }
    for (const entity of entities) {
        for (const link of entity.links) {
            if (graph.hasEdge(entity.uuid, link.source.uuid)) continue;
            graph.addEdge(entity.uuid, link.source.uuid);
            // graph.addEdge(entity.uuid, link.target.uuid);
        }
    }

    console.log('graph', graph)

    // With settings:
    const positions = forceLayout(graph, {
        maxIterations: 50,
        // settings: {
        //     gravity: 10
        // }
    });

    // To directly assign the positions to the nodes:
    // forceLayout.assign(graph);

    console.log('positions final', positions);
}