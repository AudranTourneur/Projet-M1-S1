use std::collections::HashMap;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    containers::{get_all_containers, Container},
    sqlitedb::get_sqlite_connection,
};

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export)]
pub struct Position {
    x: i32,
    y: i32,
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

use sqlx::Row;

async fn retrieve_container_positions_from_database() -> HashMap<String, (i32, i32)> {
    let mut conn = get_sqlite_connection().await.unwrap();

    let rows = sqlx::query("SELECT container_id, position_x, position_y FROM topology")
        .fetch_all(&mut conn)
        .await
        .expect("Failed to fetch topology data from database");

    let mut container_positions = HashMap::new();

    for row in rows {
        let container_id: String = row.get("container_id");
        let position_x: i32 = row.get("position_x");
        let position_y: i32 = row.get("position_y");

        container_positions.insert(container_id, (position_x, position_y));
    }

    container_positions
}

async fn create_topology_containers() -> Vec<TopologyContainer> {
    let all_containers: Vec<Container> = get_all_containers().await;

    let all_containers_positions = retrieve_container_positions_from_database().await;
    // print all
    for (key, value) in all_containers_positions.iter() {
        println!("debug {}: {:?}", key, value);
    }

    let topology_containers: Vec<TopologyContainer> = all_containers
        .iter()
        .map(|container| {
            let container_position: Option<Position> = all_containers_positions
                .get(&container.id)
                .map(|(x, y)| Position { x: *x as i32, y: *y as i32 });

            println!("obtained container_position: {:?} for {}", container_position, container.id.clone());

            let container_data = TopologyContainer {
                name: container.id.clone(),
                image: container.image.clone(),
                icon_url: "https://cdn.iconscout.com/icon/free/png-256/nginx-226046.png"
                    .to_string(),
                exposed_ports: vec![],
                exposed_volumes: vec![],
                connected_to: vec![],
                position: container_position,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopologyResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopologySaveInputContainer {
    pub id: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopologySaveRequest {
    pub containers: Vec<TopologySaveInputContainer>,
}

#[post("/topology/save", format = "json", data = "<input>")]
pub async fn topology_save_handler(input: Json<TopologySaveRequest>) -> Json<TopologyResponse> {
    let mut conn = get_sqlite_connection().await.unwrap();

    for container in input.containers.iter() {
        // upsert
        sqlx::query("INSERT OR REPLACE INTO topology (container_id, position_x, position_y) VALUES (?, ?, ?)")
            .bind(container.id.clone())
            .bind(container.x)
            .bind(container.y)
            .execute(&mut conn)
            .await
            .expect("Failed to insert topology data into database");
    }
    Json(TopologyResponse { ok: true })
}
