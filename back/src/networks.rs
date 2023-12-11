use bollard::Docker;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    id: String,
    name: String,
    created: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkResponse {
    networks: Vec<Network>,
}

async fn get_all_networks() ->  Vec<Network>{
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let base_networks = &docker.list_networks::<String>(None).await.unwrap();

    let my_networks: Vec<Network> = base_networks
        .iter()
        .map(|network| {
            network.clone().containers.clone().unwrap_or_default();
            let image_data = Network {
                id: network.id.clone().unwrap_or("UNDEFINED".to_string()),
                name: network.name.clone().unwrap_or("UNDEFINED".to_string()),
                created: network.created.clone().unwrap_or("UNDEFINED".to_string()),
            };
            image_data
        })
        .collect();

    my_networks
}

#[get("/networks")]
pub async fn networks_handler() -> Json<NetworkResponse> {
    let my_networks = get_all_networks().await;

    let response = NetworkResponse {
        networks: my_networks,
    };

    Json(response)
}

#[get("/networks/<id>")]
pub async fn network_handler(id: &str) -> Json<Network> {
    let all_networks : Vec<Network> = get_all_networks().await;
    let network = all_networks.iter().find(|network| network.id == id).unwrap();

    let response = Network {
        id: network.id.clone(),
        name: network.name.clone(),
        created: network.created.clone(),
    };

    Json(response)
}