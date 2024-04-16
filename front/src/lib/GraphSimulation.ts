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
        graph.setNodeAttribute(entity.uuid, 'size', 500);
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

    function updatePositionsForceAtlas2() {
        const sensibleSettings = forceAtlas2.inferSettings(graph);

        // sensibleSettings.adjustSizes = true;

        // console.log('sensibleSettings', sensibleSettings);

        const positions = forceAtlas2(graph, {
            iterations: 1000,
            settings: sensibleSettings,
        });

        for (const entity of entities) {
            const position = positions[entity.uuid] as { x: number, y: number; };
            if (!position) continue;
            entity.pixiContainer.x = position.x * 15;
            entity.pixiContainer.y = position.y * 15;
        }

        for (const entity of entities) {
            for (const link of entity.links) {
                link.update();
            }
        }

        // update graph
        for (const position of Object.entries(positions)) {
            const [node, { x, y }] = position;
            graph.setNodeAttribute(node, 'x', x);
            graph.setNodeAttribute(node, 'y', y);
        }

        // requestAnimationFrame(updatePositionsForceAtlas2);
    }

    updatePositionsForceAtlas2();


    function updatePositionsForce() {
        const positions = forceLayout(graph, {
            maxIterations: 1,
            settings: {
                attraction: 0.1,
                repulsion: 0.1,
                gravity: 0.1,
            }
        });

        for (const entity of entities) {
            const position = positions[entity.uuid] as { x: number, y: number; };
            if (!position) continue;
            entity.pixiContainer.x = position.x * 10;
            entity.pixiContainer.y = position.y * 10;
        }

        for (const entity of entities) {
            for (const link of entity.links) {
                link.update();
            }
        }

        // update graph
        for (const position of Object.entries(positions)) {
            const [node, { x, y }] = position;
            graph.setNodeAttribute(node, 'x', x);
            graph.setNodeAttribute(node, 'y', y);
        }

        requestAnimationFrame(updatePositionsForce);
    }

    // updatePositionsForce();

    let updateRequired = true;

    function preventCollisionUpdate() {
        const COLLISION_DISTANCE = 500;
        // detect collisions
        for (const entity of entities) {
            for (const otherEntity of entities) {
                if (entity === otherEntity) continue;
                const distance = Math.sqrt(
                    Math.pow(entity.pixiContainer.x + entity.actualCenter.x - (otherEntity.pixiContainer.x + otherEntity.actualCenter.x), 2) +
                    Math.pow(entity.pixiContainer.y + entity.actualCenter.y - (otherEntity.pixiContainer.y + otherEntity.actualCenter.y), 2)
                );
                if (distance < COLLISION_DISTANCE) {
                    console.log('collision', entity.uuid, otherEntity.uuid, distance)
                    const vector = {
                        x: entity.pixiContainer.x + entity.actualCenter.x - (otherEntity.pixiContainer.x + otherEntity.actualCenter.x),
                        y: entity.pixiContainer.y + entity.actualCenter.y - (otherEntity.pixiContainer.y + otherEntity.actualCenter.y),
                    }
                    const magnitude = Math.sqrt(vector.x * vector.x + vector.y * vector.y);
                    const unitVector = {
                        x: vector.x / magnitude,
                        y: vector.y / magnitude,
                    }

                    if (!entity.isDragging) {
                        entity.pixiContainer.x += unitVector.x * 1;
                        entity.pixiContainer.y += unitVector.y * 1;
                    }

                    if (!otherEntity.isDragging) {
                        otherEntity.pixiContainer.x -= unitVector.x * 1;
                        otherEntity.pixiContainer.y -= unitVector.y * 1;
                    }
                    updateRequired = true;
                }
            }
        }
    }

    function springSimulationUpdate() {
        const SPRING_CONSTANT = 0.005;
        for (const entity of entities) {
            for (const otherEntity of entities) {
                if (entity === otherEntity) continue;
                if (!entity.links.find(link => link.source === otherEntity || link.target === otherEntity)) continue;
                const distance = Math.sqrt(
                    Math.pow(entity.pixiContainer.x + entity.actualCenter.x - (otherEntity.pixiContainer.x + otherEntity.actualCenter.x), 2) +
                    Math.pow(entity.pixiContainer.y + entity.actualCenter.y - (otherEntity.pixiContainer.y + otherEntity.actualCenter.y), 2)
                );

                // const maxAllowedDistance = 700;
                // const maxAllowedDistance = entity.links.length * 200;
                const maxAllowedDistance = Math.min(700, Math.max(entity.links.length * 100, otherEntity.links.length * 100));

                if (distance > maxAllowedDistance) {
                    const force = SPRING_CONSTANT * distance;
                    const vector = {
                        x: entity.pixiContainer.x + entity.actualCenter.x - (otherEntity.pixiContainer.x + otherEntity.actualCenter.x),
                        y: entity.pixiContainer.y + entity.actualCenter.y - (otherEntity.pixiContainer.y + otherEntity.actualCenter.y),
                    }
                    const magnitude = Math.sqrt(vector.x * vector.x + vector.y * vector.y);
                    const unitVector = {
                        x: vector.x / magnitude,
                        y: vector.y / magnitude,
                    }
                    if (!entity.isDragging) {
                        entity.pixiContainer.x -= unitVector.x * force;
                        entity.pixiContainer.y -= unitVector.y * force;
                    }

                    if (!otherEntity.isDragging) {
                        otherEntity.pixiContainer.x += unitVector.x * force;
                        otherEntity.pixiContainer.y += unitVector.y * force;
                    }
                    updateRequired = true;
                }

            }
        }
    }

    function areTwoNodesInTheSameNetwork(node1: TopologyEntityPixi, node2: TopologyEntityPixi): boolean {
        let allNodesJoinableFromNode1 = [node1];
        let allNodesJoinableFromNode2 = [node2];
        let visitedNodes = new Set();
        let found = false;

        while (allNodesJoinableFromNode1.length > 0 || allNodesJoinableFromNode2.length > 0) {
            if (allNodesJoinableFromNode1.length > 0) {
                const node = allNodesJoinableFromNode1.pop();
                if (node === node2) {
                    found = true;
                    break;
                }
                if (visitedNodes.has(node)) continue;
                visitedNodes.add(node);
                if (!node) {
                    throw new Error('Node is null');
                }
                for (const link of node.links) {
                    if (link.source === node) {
                        allNodesJoinableFromNode1.push(link.target);
                    } else {
                        allNodesJoinableFromNode1.push(link.source);
                    }
                }
            }

            if (allNodesJoinableFromNode2.length > 0) {
                const node = allNodesJoinableFromNode2.pop();
                if (node === node1) {
                    found = true;
                    break;
                }
                if (visitedNodes.has(node)) continue;
                visitedNodes.add(node);
                if (!node) {
                    throw new Error('Node is null');
                }
                for (const link of node.links) {
                    if (link.source === node) {
                        allNodesJoinableFromNode2.push(link.target);
                    } else {
                        allNodesJoinableFromNode2.push(link.source);
                    }
                }
            }
        }

        return found;
    }

    function gravityRepulsionSimulationUpdate() {
        // each entity is repelled by each other entity
        const REPULSION_CONSTANT = -100000;
        const MAX_DISTANCE = 1200;

        for (const entity of entities) {
            for (const otherEntity of entities) {
                if (entity === otherEntity) continue;
                const distance = Math.sqrt(
                    Math.pow(entity.pixiContainer.x + entity.actualCenter.x - (otherEntity.pixiContainer.x + otherEntity.actualCenter.x), 2) +
                    Math.pow(entity.pixiContainer.y + entity.actualCenter.y - (otherEntity.pixiContainer.y + otherEntity.actualCenter.y), 2)
                );

                if (distance > MAX_DISTANCE) continue;

                let actualRepulsionConstant = REPULSION_CONSTANT;

                if (!areTwoNodesInTheSameNetwork(entity, otherEntity)) {
                    actualRepulsionConstant = REPULSION_CONSTANT * 10;
                }

                const force = actualRepulsionConstant / (distance * distance);
                const vector = {
                    x: entity.pixiContainer.x + entity.actualCenter.x - (otherEntity.pixiContainer.x + otherEntity.actualCenter.x),
                    y: entity.pixiContainer.y + entity.actualCenter.y - (otherEntity.pixiContainer.y + otherEntity.actualCenter.y),
                }

                if (Math.random() < 0.001) {
                    console.log('vector', vector, 'distance', distance, 'force', force, 'REPULSION_CONSTANT', REPULSION_CONSTANT, 'distance', distance, 'entity', entity.uuid, 'otherEntity', otherEntity.uuid)
                }
                const magnitude = Math.sqrt(vector.x * vector.x + vector.y * vector.y);
                const unitVector = {
                    x: vector.x / magnitude,
                    y: vector.y / magnitude,
                }
                if (!entity.isDragging) {
                    entity.pixiContainer.x -= unitVector.x * force;
                    entity.pixiContainer.y -= unitVector.y * force;
                }

                if (!otherEntity.isDragging) {
                    otherEntity.pixiContainer.x += unitVector.x * force;
                    otherEntity.pixiContainer.y += unitVector.y * force;
                }
                updateRequired = true;
            }
        }
    }

    function centralGravitySimulationUpdate() {
        const GRAVITY_CONSTANT = 0.001;
        const CENTER_X = 0;
        const CENTER_Y = 0;

        for (const entity of entities) {
            const vector = {
                x: entity.pixiContainer.x + entity.actualCenter.x - CENTER_X,
                y: entity.pixiContainer.y + entity.actualCenter.y - CENTER_Y,
            }
            const magnitude = Math.sqrt(vector.x * vector.x + vector.y * vector.y);
            const unitVector = {
                x: vector.x / magnitude,
                y: vector.y / magnitude,
            }
            const force = GRAVITY_CONSTANT * magnitude;
            if (!entity.isDragging) {
                entity.pixiContainer.x -= unitVector.x * force;
                entity.pixiContainer.y -= unitVector.y * force;
            }
        }
    }

    setInterval(() => {
        springSimulationUpdate();
    }, 20)

    setInterval(() => {
        gravityRepulsionSimulationUpdate();
    }, 20)

    setInterval(() => {
        centralGravitySimulationUpdate();
    }, 20)

    // setInterval(() => {
    //     preventCollisionUpdate();
    // }, 20)

    setInterval(() => {
        if (!updateRequired) return;
        for (const entity of entities) {
            for (const link of entity.links) {
                link.update();
            }
        }
    }, 20)



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