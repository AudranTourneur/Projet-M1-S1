use std::{fs::File, io::{Read}};

use bollard::{container::ListContainersOptions, secret::PortTypeEnum, Docker};
use futures::future::join_all;
use serde_yaml::{from_str, Value};

use crate::{docker::get_docker_socket, icons::resolve_icon_url_from_image_name};

use super::models::{ContainerData, OurPortTypeEnum, PortData};

pub async fn get_container_by_id(id: &str) -> Option<ContainerData> {
    let docker: Docker = get_docker_socket();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await.unwrap_or(Vec::new());

    let container = containers
        .iter()
        .find(|container| {
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

    let endpoint_settings =  network_settings.networks.clone();
    let endpoint_settings = match endpoint_settings {
        Some(endpoint_settings) => endpoint_settings,
        None => return None,
    };

    let networks: Vec<String> = endpoint_settings.keys().cloned().collect();

    let image_name = container.image.clone();
    let imgname = image_name.clone();
   
    let mut icon_url: Option<String> = None;

    if imgname.is_some() {
        let image_name = imgname.unwrap();
        let image_history = docker.image_history(&image_name).await;
   

        if let Ok(history) = image_history {
            for history in history {
                let image_id = history.id.clone();
                let image_id = image_id.to_string();
                icon_url = resolve_icon_url_from_image_name(&image_id).await;
                if icon_url.is_some() {
                    break;
                }
            }
        }
   
    }

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



pub fn yaml_read(yaml_path : String) -> Result<String, Box<dyn std::error::Error>> {
    println!("yaml_read A");
    let path = format!("/rootfs/{}",yaml_path);
    println!("path reminder {:?}", path.clone());
    let mut file = File::open(path)?;
    println!("yaml_read B");
    let mut contents = String::new();
    let file_to_string = file.read_to_string(&mut contents)?;
    println!("yaml_read C{}", contents);

    Ok(contents)
}

pub async fn modify_container_yml(id: &str){ //rebind: ContainerPortRebind
    let my_cont_data = get_container_by_id(id).await;
    let yml_path = my_cont_data.unwrap().compose_file;
    let path_string = yml_path.clone().unwrap_or_default();
    //ex : /home/abyuka/Documents/Projet-M1-S1/docker-compose.yml
    if path_string.is_empty(){
        println!("No docker compose found.")
    }
    else {
        println!("Docker compose found at : {:?}", path_string.clone());
        yaml_read(path_string);
    }
}