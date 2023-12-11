use bollard::Docker;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpamConfig {
    subnet: String,
    ip_range: String,
    gateway: String,
    aux_addresses: Option<HashMap<String, String, RandomState>>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    id: String,
    name: String,
    created: String,
    labels : Option<HashMap<String, String, RandomState>>,
    ipam_config: Option<Vec<IpamConfig>>
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
                labels: None,
                ipam_config: None,
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

    let config: Vec<IpamConfig> = network.ipam_config.unwrap();

    let config: Vec<IpamConfig> = config.iter().map(|config| {
        let config = IpamConfig {
            subnet: config.subnet,
            ip_range: config.ip_range,
            gateway: config.gateway,
            aux_addresses: config.aux_addresses
        };
        config
    }).collect();

    let response = Network {
        id: network.id.clone(),
        name: network.name.clone(),
        created: network.created.clone(),
        labels: network.labels.clone(),
        ipam_config: Some(config),
    };

    Json(response)
}