use std::{fs::File, io::Read};

use bollard::{container::ListContainersOptions, secret::PortTypeEnum, Docker};
use futures::future::join_all;

use crate::docker::get_docker_socket;

use super::models::{ContainerData, OurPortTypeEnum, PortData};

use crate::images::common::get_image_by_id;

pub async fn get_container_by_id(id: &str) -> Option<ContainerData> {
    let docker: Docker = get_docker_socket();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap_or(Vec::new());

    let container = containers.iter().find(|container| {
        let container_id = container.id.clone();
        let container_id = match container_id {
            Some(container_id) => container_id,
            None => return false,
        };
        container_id == id
    });

    let container = match container {
        Some(container) => container,
        None => return None,
    };

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
    let compose_file = labels
        .get("com.docker.compose.project.config_files")
        .map(|x| x.to_string());

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
    let raw_data = docker.inspect_container(&id, None).await;
    let raw_data = match raw_data {
        Ok(raw_data) => raw_data,
        Err(_) => return None,
    };

    let raw_data = serde_json::to_string_pretty(&raw_data);
    let raw_data = match raw_data {
        Ok(raw_data) => raw_data,
        Err(_) => return None,
    };

    let network_settings = container.network_settings.clone();
    let network_settings = match network_settings {
        Some(network_settings) => network_settings,
        None => return None,
    };

    let endpoint_settings = network_settings.networks.clone();
    let endpoint_settings = match endpoint_settings {
        Some(endpoint_settings) => endpoint_settings,
        None => return None,
    };

    let networks: Vec<String> = endpoint_settings.keys().cloned().collect();

    let image_data = match &container.image_id {
        Some(id) => get_image_by_id(&id).await,
        None => None,
    };

    let icon_url: Option<String> = match image_data {
        Some(image_data) => image_data.icon_url,
        None => None,
    };

    let container_data = ContainerData {
        icon_url,
        id: container.id.clone().unwrap_or("".into()),
        names: container.names.clone().unwrap_or(vec![]),
        image: container.image.clone().unwrap_or("".into()),
        volumes,
        networks,
        status: container.status.clone().unwrap_or("".into()),
        ports,
        compose_file,
        labels: Some(labels),
        raw_data: Some(raw_data),
    };

    Some(container_data)
}

pub async fn get_all_containers() -> Vec<ContainerData> {
    let docker: Docker = get_docker_socket();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap_or(vec![]);

    let listed_containers_futures = containers.iter().map(|container| async {
        let container_id = container.id.clone();
        let container_id = match container_id {
            Some(container_id) => container_id,
            None => return None,
        };
        let result = get_container_by_id(&container_id).await;
        result
    });

    let listed_containers = join_all(listed_containers_futures).await;

    let listed_containers: Vec<ContainerData> = listed_containers
        .into_iter()
        .filter_map(|container| container)
        .collect();

    listed_containers
}

pub fn yaml_string(yaml_path: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("yaml_read A");
    let path = format!("/rootfs/{}", yaml_path);
    println!("path reminder {:?}", path.clone());
    let mut file = File::open(path)?;
    println!("yaml_read B");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("yaml_read C{}", contents);

    Ok(contents)
}

pub async fn modify_container_yml(id: &str) {
    //, port, new_port
    println!("Called modify_container_yml with id: {:?}", id);
    //rebind: ContainerPortRebind
    let container_data = get_container_by_id(id).await.unwrap();
    let yml_path = container_data.clone().compose_file;
    let path_string = yml_path.clone().unwrap_or_default();
    //ex : /home/abyuka/Documents/Projet-M1-S1/docker-compose.yml

    if path_string.is_empty() {
        println!("No docker compose found.");
        return;
    }

    println!("Docker compose found at : {:?}", path_string.clone());
    let docker_compose_str = yaml_string(path_string);

    let docker_compose_str = match docker_compose_str {
        Ok(docker_compose_str) => docker_compose_str,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    let docker_compose: serde_yaml::Value = serde_yaml::from_str(&docker_compose_str).unwrap();
    let dc_services = docker_compose["services"].as_mapping().unwrap();
    println!("Docker compose services: {:?}", dc_services.clone());

    let labels = container_data.clone().labels;

    let key = "com.docker.compose.service";

    let service_name = match labels {
        Some(labels) => match labels.get(key) {
            Some(service_name) => service_name.clone(),
            None => {
                println!("No service name found in labels.");
                return;
            }
        },
        None => {
            println!("No labels found.");
            return;
        }
    };

    let service = dc_services.get(&serde_yaml::Value::String(service_name.clone()));
    let service = match service {
        Some(service) => service,
        None => {
            let all_available_keys: Vec<&serde_yaml::Value> = dc_services.keys().collect();
            println!(
                "Failed to get service, available keys: {:?}",
                all_available_keys
            );
            return;
        }
    };

    let ports = service.get("ports");
    let ports = match ports {
        Some(ports) => ports,
        None => {
            let all_available_keys: Vec<&serde_yaml::Value> = dc_services.keys().collect();
            println!(
                "Failed to get ports, available keys: {:?}",
                all_available_keys
            );
            return;
        }
    };

    println!("Ports: {:?}", ports.clone());
}

// récupere PORTS ok
// route rebind ports change le docker compose avec les ports voulus YUMP
