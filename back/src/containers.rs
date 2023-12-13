use bollard::container::StartContainerOptions;
use bollard::container::StopContainerOptions;
use bollard::{container::ListContainersOptions, Docker};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    id: String,
    name: Vec<String>,
    image: String,
    network: String,
    volume: Vec<String>,
    status: String,
    ports: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerList {
    containers: Vec<Container>,
}

#[get("/containers")]
pub async fn containers_handler() -> Json<ContainerList> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let ran_string = "test";

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
                ports: ran_string.to_string(),
            };
            container_data
        })
        .collect();

    let res = ContainerList {
        containers: listed_containers,
    };

    Json(res)
}

#[get("/containers/<id>")]
pub async fn container_handler(id: &str) -> Json<Container> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let ran_string = "test";

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
        ports: ran_string.to_string(),
    };

    Json(container_data)
}

#[post("/containers/<id>/start")]
pub async fn container_start(id: &str) -> &'static str {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

    let start_options: StartContainerOptions<String> = StartContainerOptions::default();

    match docker.start_container(&id, Some(start_options)).await {
        Ok(_) => "Container started",
        Err(_) => "Error starting container",
    }
}

#[post("/containers/<id>/stop")]
pub async fn container_stop(id: &str) -> &'static str {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

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
