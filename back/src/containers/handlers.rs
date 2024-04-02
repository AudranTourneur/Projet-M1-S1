use bollard::{container::StartContainerOptions, Docker};
use bollard::container::{RemoveContainerOptions, StopContainerOptions};
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;

use crate::docker::get_docker_socket;

use super::common::{get_all_containers, get_container_by_id};
use super::models::{ContainerData, ContainerList, ContainerStatsResponse};



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

#[post("/containers/<id>/rebind-ports")]
pub async fn rebind_ports_handler(id: &str) -> String {
    format!("WIP {}", id)
}

#[get("/containers/<id>/filesystem")]
pub async fn container_filesystem_handler(id: &str) -> String {
    format!("WIP {}", id)
}

#[get("/container/<id>")]
pub async fn container_handler(id: &str) -> Json<ContainerData> {
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

#[get("/containers")]
pub async fn containers_handler() -> Json<ContainerList> {
    let listed_containers = get_all_containers().await;

    let res = ContainerList {
        containers: listed_containers,
    };

    Json(res)
}

