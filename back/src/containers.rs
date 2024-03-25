use bollard::container::StartContainerOptions;
use bollard::container::StopContainerOptions;
use bollard::{container::ListContainersOptions, Docker};
use bollard::models::Port;
use rocket::response::stream::TextStream;
use rocket::serde::{json::Json, Deserialize, Serialize};
use bollard::container::RemoveContainerOptions;

use crate::docker::get_docker_socket;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub id: String,
    pub name: Vec<String>,
    pub image: String,
    pub network: String,
    pub volume: Vec<String>,
    pub status: String,
    pub ports: Vec<Port>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerList {
    containers: Vec<Container>,
}

pub async fn get_all_containers() -> Vec<Container> {
 let docker: Docker = get_docker_socket();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let listed_containers: Vec<Container> = containers
        .iter()
        .map(|container| {
            let volume_name: Vec<String> = container
                .mounts
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|volume| volume.name.clone().unwrap_or_default())
                .collect();


            let container_port = container.ports.clone().unwrap_or_default();
            println!("container_port: {:?}", container_port);

            let container_data = Container {
                id: container.id.clone().unwrap_or("UNDEFINED".to_string()),
                name: container.names.clone().unwrap(),
                image: container.image.clone().unwrap(),
                volume: volume_name,
                network: container
                    .network_settings
                    .clone()
                    .unwrap()
                    .networks
                    .clone()
                    .unwrap()
                    .keys()
                    .cloned()
                    .collect(),
                status: container.status.clone().unwrap(),
                ports:  container.ports.clone().unwrap_or_default()
            };
            container_data
        })
        .collect();

    listed_containers
}

#[get("/containers")]
pub async fn containers_handler() -> Json<ContainerList> {
    let listed_containers = get_all_containers().await;

    let res = ContainerList {
        containers: listed_containers,
    };

    Json(res)
}

#[get("/container/<id>")]
pub async fn container_handler(id: &str) -> Json<Container> {
    let docker: Docker = get_docker_socket();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();



    let container = containers
        .iter()
        .find(|container| container.id.clone().unwrap() == id)
        .unwrap();

    let volume_name: Vec<String> = container
        .mounts
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|volume| volume.name.clone().unwrap_or_default())
        .collect();

    let container_data = Container {
        id: container.id.clone().unwrap_or("UNDEFINED".to_string()),
        name: container.names.clone().unwrap(),
        image: container.image.clone().unwrap(),
        volume: volume_name,
        network: container
            .network_settings
            .clone()
            .unwrap()
            .networks
            .clone()
            .unwrap()
            .keys()
            .cloned()
            .collect(),
        status: container.status.clone().unwrap(),
        ports:   container.ports.clone().unwrap_or_default()
    };

    Json(container_data)
}

#[post("/container/<id>/start")]
pub async fn container_start(id: &str) -> &'static str {
    let docker: Docker = get_docker_socket();

    let start_options: StartContainerOptions<String> = StartContainerOptions::default();

    match docker.start_container(&id, Some(start_options)).await {
        Ok(_) => "Container started",
        Err(_) => "Error starting container",
    }
}

#[post("/container/<id>/stop")]
pub async fn container_stop(id: &str) -> &'static str {
    let docker: Docker = get_docker_socket();

    let options = Some(StopContainerOptions { t: 30 });

    match docker.stop_container(&id, options).await {
        Ok(_) => "Container stopped",
        Err(_) => "Error stopping container",
    }
}

#[derive(Serialize, Debug)]
pub struct ContainerStatsResponse {
    pub stats: Vec<crate::database::MyRow>,
}

#[get("/statistics-historical/container/<id>")]
pub async fn container_stats_handler(id: &str) -> Json<ContainerStatsResponse> {
    let db_res = crate::database::get_historical_statistics_for_container(id.to_string()).await;

    match db_res {
        Ok(stats) => {
            println!("Stats: {:?}", stats);
            let res = ContainerStatsResponse { stats };

            Json(res)
        }
        Err(e) => {
            println!("Error getting stats: {}", e);

            let res = ContainerStatsResponse { stats: vec![] };

            Json(res)
        }
    }
}

#[get("/statistics-realtime/container/<_id>")]
pub async fn container_stats_stream_hander(_id: &str) -> TextStream![String] {
    TextStream! {
        for id in 0..42 {
            // yield with the counter
            yield format!("{}\n", id);
            // sleep async for 100ms
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}


#[post("/containers/<id>/remove")]
pub async fn delete_container(id: &str) -> &'static str{
    let docker : Docker = get_docker_socket();
    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    let _ = docker.remove_container(id, options).await;

    "Success."
}
