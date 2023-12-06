use bollard::Docker;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct Network {
    id: String,
    name: String,
    created: String,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkResponse {
    networks: Vec<Network>
}

#[get("/networks")]
pub async fn networks_handler() -> Json<NetworkResponse> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

    let base_networks = &docker.list_networks::<String>(None).await.unwrap();

    let my_networks : Vec<Network> = base_networks.iter().map(|network| {
        network.clone().containers.clone().unwrap_or_default();
        let image_data = Network {
            id: network.id.clone().unwrap_or("UNDEFINED".to_string()),
            name: network.name.clone().unwrap_or("UNDEFINED".to_string()),
            created: network.created.clone().unwrap_or("UNDEFINED".to_string()),
        };
        image_data
    }).collect();

    let response = NetworkResponse {
        networks: my_networks,
    };

    Json(response)
}
