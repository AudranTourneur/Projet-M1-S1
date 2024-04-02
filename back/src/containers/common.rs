use bollard::{container::ListContainersOptions, secret::PortTypeEnum, Docker};
use futures::future::join_all;

use crate::docker::get_docker_socket;

use super::models::{ContainerData, PortData, OurPortTypeEnum};

pub async fn get_container_by_id(id: &str) -> ContainerData {
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

    let volumes: Vec<String> = container
        .mounts
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|volume| {
            let name = volume.name.clone().unwrap_or_default();
            if name.len() > 0 {
                return name;
            }
            return volume.source.clone().unwrap_or_default();
            
        })
        .collect();

    let labels = container.labels.clone().unwrap_or_default();
    let compose_file = labels.get("com.docker.compose.project.config_files").map(|x| x.to_string());

    let ports: Vec<PortData> = container
        .ports
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|port| {
            let our_port = PortData {
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

    let networks: Vec<String> = container.network_settings.clone().unwrap().networks.clone().unwrap().keys().cloned().collect();

    let is_running: bool = container.status.clone().unwrap().starts_with("Up");

    let container_data = ContainerData {
        icon_url: None,
        id: container.id.clone().unwrap_or("UNDEFINED".to_string()),
        names: container.names.clone().unwrap(),
        image: container.image.clone().unwrap(),
        volumes,
        networks,
        status: container.status.clone().unwrap(),
        ports,
        compose_file,
        labels: Some(labels),
        raw_data: Some(raw_data),
        is_running,
    };

    container_data
}

pub async fn get_all_containers() -> Vec<ContainerData> {
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

