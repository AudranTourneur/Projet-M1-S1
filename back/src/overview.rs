use bollard::{image::ListImagesOptions, Docker};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverviewResponse {
    version_docker: String,
    version_linux: String,
    images: i32,
    containers: i32,
    volumes: i32,
    networks: i32,
}

#[get("/overview")]
pub async fn overview_handler() -> Json<OverviewResponse> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

    let version_object = docker.version().await.unwrap();
    let linux_version = version_object
        .kernel_version
        .unwrap_or("UNDEFINED".to_string());
    let docker_version = version_object.version.unwrap_or("UNDEFINED".to_string());

    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let number_of_images = images.len() as i32;

    let containers = &docker.list_containers::<String>(None).await.unwrap();
    let number_of_containers = containers.len() as i32;

    let volume_response = &docker.list_volumes::<String>(None).await.unwrap();
    let number_of_volumes = volume_response.volumes.clone().unwrap().len() as i32;

    let networks = &docker.list_networks::<String>(None).await.unwrap();
    let number_of_networks = networks.len() as i32;

    let response = OverviewResponse {
        version_docker: docker_version,
        version_linux: linux_version,
        images: number_of_images,
        containers: number_of_containers,
        volumes: number_of_volumes,
        networks: number_of_networks,
    };

    Json(response)
}
