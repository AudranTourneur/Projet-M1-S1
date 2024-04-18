use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    auth::JWT,
    containers::{common::get_all_containers, models::ContainerData},
    images::{common::get_all_images, models::ImageData},
    ports::{get_used_ports, SimplePortData},
    sqlitedb::get_sqlite_connection,
    state::AllVolumesCacheState,
    volumes::models::VolumeData,
};

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopologyContainer {
    pub data: ContainerData,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopologyPort {
    pub data: SimplePortData,
    pub connected_to: Vec<String>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopologyVolume {
    pub data: VolumeData,
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
    pub images: Vec<ImageData>,
    pub clusters: Vec<Vec<TopologyContainer>>,
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
    let all_containers: Vec<ContainerData> = get_all_containers().await;

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
                .map(|(x, y)| Position {
                    x: *x as i32,
                    y: *y as i32,
                });

            println!(
                "obtained container_position: {:?} for {}",
                container_position,
                container.id.clone()
            );

            TopologyContainer {
                data: container.clone(),
                connected_to: vec![],
                position: container_position,
            }
        })
        .collect();

    topology_containers
}

async fn create_topology_clusters() -> Vec<Vec<TopologyContainer>> {
    vec![]
}

fn create_topology_ports() -> Vec<TopologyPort> {
    let used_ports = get_used_ports();
    let used_ports = match used_ports {
        Ok(used_ports) => used_ports,
        Err(e) => {
            println!("Error getting ports: {}", e);
            Vec::new()
        }
    };

    let final_ports = used_ports
        .iter()
        .map(|port| TopologyPort {
            data: port.clone(),
            connected_to: vec![],
            position: None,
        })
        .collect();

    final_ports
}

fn create_topology_volumes(volumes: Vec<VolumeData>) -> Vec<TopologyVolume> {
    let topology_volumes: Vec<TopologyVolume> = volumes
        .iter()
        .map(|data| TopologyVolume {
            data: data.clone(),
            connected_to: vec![],
            position: None,
        })
        .collect();

    topology_volumes
}

#[get("/topology")]
pub async fn topology_handler(
    _key: JWT,
    volume_state: &rocket::State<Arc<Mutex<AllVolumesCacheState>>>,
) -> Json<Topology> {
    let volumes_data = {
        let state = volume_state.lock().unwrap();
        state.volumes.clone()
    };

    let topo = Topology {
        containers: create_topology_containers().await,
        ports: create_topology_ports(),
        volumes: create_topology_volumes(volumes_data),
        images: get_all_images().await,
        clusters: create_topology_clusters().await,
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
pub async fn topology_save_handler(
    _key: JWT,
    input: Json<TopologySaveRequest>,
) -> Json<TopologyResponse> {
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
