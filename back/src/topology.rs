use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::containers::{get_all_containers, Container};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopologyContainer {
    pub name: String,
    pub image: String,
    pub icon_url: String,
    pub exposed_ports: Vec<u16>,
    pub exposed_volumes: Vec<String>,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopologyPort {
    pub interface: String,
    pub number: u16,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopologyVolume {
    pub name: String,
    pub size: u64,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Topology {
    pub containers: Vec<TopologyContainer>,
    pub ports: Vec<TopologyPort>,
    pub volumes: Vec<TopologyVolume>,
    pub position: Option<Position>,
}

async fn create_topology_containers() -> Vec<TopologyContainer> {
    let all_containers: Vec<Container> = get_all_containers().await;

    let topology_containers: Vec<TopologyContainer> = all_containers
        .iter()
        .map(|container| {
            let container_data = TopologyContainer {
                name: container.id.clone(),
                image: container.image.clone(),
                icon_url: "https://cdn.iconscout.com/icon/free/png-256/nginx-226046.png"
                    .to_string(),
                exposed_ports: vec![],
                exposed_volumes: vec![],
                connected_to: vec![],
                position: None,
            };
            container_data
        })
        .collect();

    topology_containers
}

fn create_topology_ports() -> Vec<TopologyPort> {
    vec![
        TopologyPort {
            interface: "eth0".to_string(),
            number: 80,
            connected_to: vec!["nginx".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
        TopologyPort {
            interface: "eth0".to_string(),
            number: 3306,
            connected_to: vec!["mysql".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
    ]
}

fn create_topology_volumes() -> Vec<TopologyVolume> {
    vec![
        TopologyVolume {
            name: "nginx".to_string(),
            size: 100,
            connected_to: vec!["nginx".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
        TopologyVolume {
            name: "mysql".to_string(),
            size: 100,
            connected_to: vec!["mysql".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
        TopologyVolume {
            name: "postgres".to_string(),
            size: 100,
            connected_to: vec!["nginx".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
        TopologyVolume {
            name: "node".to_string(),
            size: 100,
            connected_to: vec!["nginx".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
        TopologyVolume {
            name: "python".to_string(),
            size: 100,
            connected_to: vec!["nginx".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
        TopologyVolume {
            name: "ruby".to_string(),
            size: 100,
            connected_to: vec!["nginx".to_string()],
            position: Some(Position { x: 100, y: 100 }),
        },
    ]
}

#[get("/topology")]
pub async fn topology_handler() -> Json<Topology> {
    let topo = Topology {
        containers: create_topology_containers().await,
        ports: create_topology_ports(),
        volumes: create_topology_volumes(),
        position: Some(Position { x: 100, y: 100 }),
    };

    Json(topo)
}
