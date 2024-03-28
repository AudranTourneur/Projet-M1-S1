use bollard::Docker;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use bollard::models::Network;
use futures_util::future::join_all;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OurIpamConfig {
    subnet: String,
    ip_range: String,
    gateway: String,
    aux_addresses: Option<HashMap<String, String, RandomState>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkContainers {
    name: String,
    endpoint_id: String,
    mac_address: String,
    ipv4_address: String,
    ipv6_address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OurNetwork {
    id: String,
    name: String,
    created: String,
    labels: Option<HashMap<String, String, RandomState>>,
    ipam_config: Option<Vec<OurIpamConfig>>,
    containers: Option<HashMap<String, NetworkContainers>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkResponse {
    networks: Vec<OurNetwork>,
}

fn get_ipam(network_response: Network) -> Vec<OurIpamConfig> {

    let ipam_configs = network_response
        .clone()
        .ipam
        .unwrap()
        .clone()
        .config
        .unwrap();

    let config: Vec<OurIpamConfig> = ipam_configs
        .iter()
        .map(|cfg| {
            let config = OurIpamConfig {
                subnet: cfg.subnet.clone().unwrap_or_default(),
                ip_range: cfg.ip_range.clone().unwrap_or_default(),
                gateway: cfg.gateway.clone().unwrap_or_default(),
                aux_addresses: cfg.auxiliary_addresses.clone(),
            };
            config
        })
        .collect();

    config
}
fn get_containers(network_response: Network) -> HashMap<String, NetworkContainers> {

    let network_containers = network_response
        .clone()
        .containers
        .unwrap()
        .clone();

    let containers: HashMap<String, NetworkContainers> = network_containers
        .iter()
        .map(|(key, value)| {
            let container = NetworkContainers {
                name: value.name.clone().unwrap_or_default(),
                endpoint_id: value.endpoint_id.clone().unwrap_or_default(),
                mac_address: value.mac_address.clone().unwrap_or_default(),
                ipv4_address: value.ipv4_address.clone().unwrap_or_default(),
                ipv6_address: value.ipv6_address.clone().unwrap_or_default(),
            };
            (key.clone(), container)
        })
        .collect();

    containers
}

async fn get_all_networks() -> Vec<OurNetwork> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let base_networks = &docker.list_networks::<String>(None).await.unwrap();
        let my_networks = join_all(base_networks
        .iter()
        .map(|network| async {
            let docker: Docker = Docker::connect_with_local_defaults().unwrap();
            let network_response = docker.inspect_network::<String>(network.id.as_deref().unwrap_or(""), None).await.unwrap();
            let config = get_ipam(network_response.clone());
            let containers = get_containers(network_response);

            let our_network = OurNetwork {
                id: network.id.clone().unwrap_or("UNDEFINED".to_string()),
                name: network.name.clone().unwrap_or("UNDEFINED".to_string()),
                created: network.created.clone().unwrap_or("UNDEFINED".to_string()),
                labels: network.labels.clone(),
                ipam_config: Some(config),
                containers: Some(containers),
            };
            our_network
        })
        );

    my_networks.await
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
pub async fn network_handler(id: &str) -> Json<OurNetwork> {
    let all_networks: Vec<OurNetwork> = get_all_networks().await;
    let network = all_networks
        .iter()
        .find(|network| network.id == id)
        .unwrap();

    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let network_response = docker.inspect_network::<String>(id, None).await.unwrap();

    let config = get_ipam(network_response.clone());
    let containers = get_containers(network_response);


    let response = OurNetwork {
        id: network.id.clone(),
        name: network.name.clone(),
        created: network.created.clone(),
        labels: network.labels.clone(),
        ipam_config: Some(config),
        containers: Some(containers),
    };

    Json(response)
}
