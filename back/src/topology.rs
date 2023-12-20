use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyContainer {
    pub name: String,
    pub image: String,
    pub icon_url: String,
    pub exposed_ports: Vec<u16>,
    pub exposed_volumes: Vec<String>,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyPort {
    pub interface: String,
    pub number: u16,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyVolume {
    pub name: String,
    pub size: u64,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topology {
    pub containers: Vec<TopologyContainer>,
    pub ports: Vec<TopologyPort>,
    pub volumes: Vec<TopologyVolume>,
    pub position: Option<Position>,
}

#[get("/topology")]
pub fn topology_handler() -> Json<Topology> {
    let topo = Topology {
        containers: vec![
            TopologyContainer {
                name: "nginx".to_string(),
                image: "nginx:latest".to_string(),
                icon_url: "https://cdn.iconscout.com/icon/free/png-256/nginx-226046.png"
                    .to_string(),
                exposed_ports: vec![80],
                exposed_volumes: vec!["/var/www/html".to_string()],
                connected_to: vec!["mysql".to_string()],
                position: Some(Position { x: 100, y: 100 }),
            },
            TopologyContainer {
                name: "mysql".to_string(),
                image: "mysql:latest".to_string(),
                icon_url: "https://cdn.iconscout.com/icon/free/png-256/mysql-19-1174939.png"
                    .to_string(),
                exposed_ports: vec![3306],
                exposed_volumes: vec!["/var/lib/mysql".to_string()],
                connected_to: vec!["nginx".to_string()],
                position: Some(Position { x: 100, y: 100 }),
            },
        ],
        ports: vec![
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
        ],
        volumes: vec![
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
        ],
        position: Some(Position { x: 100, y: 100 })
    };

    Json(topo)
}
