import Graph from 'graphology';
import forceLayout from 'graphology-layout-force';
import circular from 'graphology-layout/circular';
// import ForceSupervisor from 'graphology-layout-force/worker';
import noverlap from 'graphology-layout-noverlap';
import forceAtlas2 from 'graphology-layout-forceatlas2';


import type { TopologyEntityPixi } from './TopologyEntityPixi';

export async function simulatePositions(entities: TopologyEntityPixi[]) {
    const graph = new Graph();
    for (const entity of entities) {
        graph.addNode(entity.uuid);
        graph.setNodeAttribute(entity.uuid, 'x', -50 + Math.random() * 100);
        graph.setNodeAttribute(entity.uuid, 'y', -50 + Math.random() * 100);
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
    // const positions = forceLayout(graph, {
    //     maxIterations: 50,
    //     // settings: {
    //     //     gravity: 10
    //     // }
    // });

    // To directly assign the positions to the nodes:
    // forceLayout.assign(graph);

    // const positions = circular(graph);
    // const positions = noverlap(graph, {maxIterations: 200});

    const sensibleSettings = forceAtlas2.inferSettings(graph);

    const positions = forceAtlas2(graph, {
        iterations: 400,
        settings: sensibleSettings,
    });

    console.log('positions final', positions);

    console.log(positions)

    for (const entity of entities) {
        const position = positions[entity.uuid] as { x: number, y: number; };
        if (!position) continue;
        console.log('POSITION', entity.uuid, position.x, position.y)
        entity.pixiContainer.x = position.x * 10;
        entity.pixiContainer.y = position.y * 10;
    }

    for (const entity of entities) {
        for (const link of entity.links) {
            link.update();
        }
    }

    /*
    const newPositions = forceLayout(graph, {
        maxIterations: 300,
    });

    console.log('new positions', newPositions);

    for (const node of graph.nodes()) {
        const position = newPositions[node] as { x: number, y: number; };
        if (!position) continue;
        graph.setNodeAttribute(node, 'x', position.x);
        graph.setNodeAttribute(node, 'y', position.y);
    }

    const layout = new ForceSupervisor(graph, {
        onConverged: () => {
            console.log('Converged!');
        }
    });

    layout.start()
    */

    // const positions = noverlap(graph, {maxIterations: 50});


    // To directly assign the positions to the nodes:
    // noverlap.assign(graph);

    // for (const entity of entities) {
    //     const x = graph.getNodeAttribute(entity.uuid, 'x')
    //     const y = graph.getNodeAttribute(entity.uuid, 'y')
    //     if (x == null || y == null) continue;
    //     console.log('position', entity.uuid, x, y)
    //     // entity.pixiContainer.x = Number(x);
    //     // entity.pixiContainer.y = Number(y);
    // }

    // for (const entity of entities) {
    //     for (const link of entity.links) {
    //         link.update();
    //     }
    // }


    // setInterval(() => {
    //     console.log('graph', graph)

    //     graph.nodes().forEach(node => {
    //         console.log('node', node, graph.getNodeAttributes(node))
    //     })
    // }, 1000)
}