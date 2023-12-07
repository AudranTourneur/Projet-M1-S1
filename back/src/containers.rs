use rocket::serde::{Serialize, Deserialize, json::Json};
use bollard::{Docker, container::ListContainersOptions};

#[derive(Serialize, Deserialize)]
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
pub struct ContainerList {
    final_containers: Vec<Container>,
}

#[get("/containers")]
pub async fn containers_handler() -> Json<ContainerList> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let containers = &docker.list_containers::<String>(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    let ran_string = "test";

    let listed_containers: Vec<Container> = containers.iter().map(|container| {

        let container_data = Container {
            id: container.id.clone().unwrap_or("UNDEFINED".to_string()),
            name: container.names.clone().unwrap(),
            image: container.image.clone().unwrap(),
            network: container.network_settings.clone().unwrap().networks.clone().unwrap().keys().cloned().collect(),
            volume: volume_names,  //line to modify
            status: container.status.clone().unwrap(),
            ports: ran_string.to_string(),
        };
        container_data
    }).collect();

    let res = ContainerList {
        final_containers: listed_containers,
    };

    Json(res)
}

