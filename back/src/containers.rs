use bollard::container::RemoveContainerOptions;
use bollard::container::StartContainerOptions;
use bollard::container::StopContainerOptions;
use bollard::secret::EndpointSettings;
use bollard::secret::PortTypeEnum;
use bollard::{container::ListContainersOptions, Docker};
use futures::future::join_all;
use rocket::response::stream::TextStream;
use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;

use crate::docker::get_docker_socket;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum OurPortTypeEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct OurPort {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    #[serde(rename = "type")]
    pub typ: Option<OurPortTypeEnum>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]

pub struct Container {
    pub icon_url: Option<String>,
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub network: String,
    // pub networks: Vec<EndpointSettings>,
    pub volumes: Vec<String>,
    pub status: String,
    pub ports: Vec<OurPort>,
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub compose_file: Option<String>,
    pub raw_data: Option<String>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerList {
    pub containers: Vec<Container>,
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

    let listed_containers_futures = containers.iter().map(|container| async {
        let result = get_container_by_id(&container.id.clone().unwrap()).await;
        result
    });

    let listed_containers = join_all(listed_containers_futures).await;

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

pub async fn get_container_by_id(id: &str) -> Container {
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

    let labels = container.labels.clone().unwrap_or_default();
    let compose_file = labels.get("com.docker.compose.project.config_files").map(|x| x.to_string());

    let ports: Vec<OurPort> = container
        .ports
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|port| {
            let our_port = OurPort {
                ip: port.ip.clone(),
                private_port: port.private_port,
                public_port: port.public_port,
                typ: port.clone().typ.map(|typ| match typ {
                    PortTypeEnum::EMPTY => OurPortTypeEnum::EMPTY,
                    PortTypeEnum::TCP => OurPortTypeEnum::TCP,
                    PortTypeEnum::UDP => OurPortTypeEnum::UDP,
                    PortTypeEnum::SCTP => OurPortTypeEnum::SCTP,
                }),
            };
            our_port
        })
        .collect();

    // call "docker inspect <container_id>"
    let raw_data = docker.inspect_container(&id, None).await.unwrap();
    let raw_data = serde_json::to_string_pretty(&raw_data).unwrap();

    let _networks: Vec<EndpointSettings> = container.network_settings.clone().unwrap().networks.clone().unwrap().values().cloned().collect();

    // let networks_str: Vec<String> = networks.iter().map(|network| network.network_id.clone().unwrap_or_default()).collect();

    let container_data = Container {
        icon_url: None,
        id: container.id.clone().unwrap_or("UNDEFINED".to_string()),
        names: container.names.clone().unwrap(),
        image: container.image.clone().unwrap(),
        volumes: volume_name,
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
        // networks: networks,
        status: container.status.clone().unwrap(),
        ports,
        compose_file,
        labels: Some(labels),
        raw_data: Some(raw_data),
    };

    container_data
}

#[get("/container/<id>")]
pub async fn container_handler(id: &str) -> Json<Container> {
    let container = get_container_by_id(id).await;

    Json(container)
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

#[derive(Serialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerStatsResponse {
    pub stats: Vec<crate::database::ContainerStatisticsRow>,
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
pub async fn delete_container(id: &str) -> &'static str {
    let docker: Docker = get_docker_socket();
    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    let _ = docker.remove_container(id, options).await;

    "Success."
}

#[post("/containers/<_id>/rebind-ports")]
pub async fn rebind_ports_handler(_id: &str) -> &'static str {
    "WIP"
}

#[get("/containers/<_id>/filesystem")]
pub async fn container_filesystem_handler(_id: &str) -> &'static str {
    "WIP"
}