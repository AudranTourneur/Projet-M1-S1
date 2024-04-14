use std::collections::HashMap;

use bollard::container::{
    Config, CreateContainerOptions, InspectContainerOptions, RemoveContainerOptions,
    StopContainerOptions,
};
use bollard::secret::{HostConfig, PortBinding};
use bollard::{container::StartContainerOptions, Docker};
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;

use crate::containers::models::ContainerPortRebindRequest;
use crate::docker::get_docker_socket;

use super::common::{check_for_yml, get_all_containers, get_container_by_id, modify_container_yml};
use super::models::{ContainerData, ContainerList, ContainerStatsResponse};

use crate::auth::JWT;

#[get("/statistics-historical/container/<id>")]
pub async fn container_stats_handler(_key: JWT, id: &str) -> Json<ContainerStatsResponse> {
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
pub async fn container_stats_stream_hander(_key: JWT, _id: &str) -> TextStream![String] {
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
pub async fn delete_container(_key: JWT, id: &str) -> &'static str {
    let docker: Docker = get_docker_socket();
    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    let _ = docker.remove_container(id, options).await;

    "Success."
}

#[get("/containers/<id>/filesystem")]
pub async fn container_filesystem_handler(_key: JWT, id: &str) -> String {
    format!("WIP {}", id)
}

#[get("/container/<id>")]
pub async fn container_handler(_key: JWT, id: &str) -> Json<Option<ContainerData>> {
    let container = get_container_by_id(id).await;

    match container {
        Some(container) => Json(Some(container)),
        None => Json(None),
    }
}

#[post("/container/<id>/start")]
pub async fn container_start(_key: JWT, id: &str) -> &'static str {
    let docker: Docker = get_docker_socket();

    let start_options: StartContainerOptions<String> = StartContainerOptions::default();

    match docker.start_container(id, Some(start_options)).await {
        Ok(_) => "Container started",
        Err(_) => "Error starting container",
    }
}

#[post("/container/<id>/stop")]
pub async fn container_stop(_key: JWT, id: &str) -> &'static str {
    let docker: Docker = get_docker_socket();

    let options = Some(StopContainerOptions { t: 30 });

    match docker.stop_container(id, options).await {
        Ok(_) => "Container stopped",
        Err(_) => "Error stopping container",
    }
}

#[get("/containers")]
pub async fn containers_handler(_key: JWT) -> Json<ContainerList> {
    let listed_containers = get_all_containers().await;

    let res = ContainerList {
        containers: listed_containers,
    };

    Json(res)
}

#[post("/containers/<id>/rebind-ports", data = "<input>")]
pub async fn rebind_ports_handler(_key: JWT, input: Json<ContainerPortRebindRequest>, id: &str) -> Json<bool> {
    if check_for_yml(id).await == false {
        let docker: Docker = get_docker_socket();

        let res = docker
            .inspect_container(id, Some(InspectContainerOptions { size: false }))
            .await;

        let res = match res {
            Ok(res) => res,
            Err(_) => return Json(false),
        };

        let rep = docker
            .stop_container(id, Some(StopContainerOptions { t: 1 }))
            .await;
        match rep {
            Ok(_) => (),
            Err(_) => return Json(false),
        };

        let rep = docker
            .remove_container(
                id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await;
        match rep {
            Ok(_) => (),
            Err(_) => return Json(false),
        };

        // let create_options = match res.name {
        //     Some(container_name) => Some(CreateContainerOptions {
        //         name: container_name,
        //         ..Default::default()
        //     }),
        //     None => None,
        // };

        let create_options = res.name.map(|container_name| CreateContainerOptions {
            name: container_name,
            ..Default::default()
        });

        let image_name = res.image;

        let port_bindings = Some(
            input
                .ports
                .clone()
                .into_iter()
                .map(|p| {
                    let port_binding = PortBinding {
                        host_ip: Some(p.ip.into()),
                        host_port: Some(p.internal.to_string()),
                    };

                    (p.internal.to_string(), Some(vec![port_binding]))
                })
                .map(|chunk| (chunk.0, chunk.1))
                .collect::<HashMap<_, _>>(),
        );

        let host_config = HostConfig {
            port_bindings,
            ..Default::default()
        };

        let cfg = Config {
            image: image_name,
            host_config: Some(host_config),
            ..Default::default()
        };

        let rep = docker.create_container(create_options, cfg).await;
        match rep {
            Ok(_) => (),
            Err(_) => return Json(false),
        };

        let rep = docker
            .start_container(id, None::<StartContainerOptions<String>>)
            .await;
        match rep {
            Ok(_) => (),
            Err(_) => return Json(false),
        };

        Json(true)
    } else {
        modify_container_yml(id, input).await;
        Json(true)
    }
}
