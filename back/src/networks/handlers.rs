use bollard::Docker;
use rocket::serde::json::Json;

use crate::docker::get_docker_socket;

use super::{
    common::{get_all_networks, get_containers, get_ipam},
    models::{NetworkData, NetworkList},
};

#[get("/networks")]
pub async fn networks_handler() -> Json<NetworkList> {
    let my_networks = get_all_networks().await;

    let response = NetworkList {
        networks: my_networks,
    };

    Json(response)
}

#[get("/networks/<id>")]
pub async fn network_handler(id: &str) -> Json<Option<NetworkData>> {
    let all_networks: Vec<NetworkData> = get_all_networks().await;
    let network = all_networks.iter().find(|network| network.id == id);
    let network = match network {
        Some(network) => network,
        None => return Json(None),
    };

    let docker: Docker = get_docker_socket();
    let network_response = docker.inspect_network::<String>(id, None).await;
    let network_response = match network_response {
        Ok(network_response) => network_response,
        Err(_) => return Json(None),
    };

    let config = get_ipam(network_response.clone());
    let containers = get_containers(network_response);

    let response = NetworkData {
        id: network.id.clone(),
        name: network.name.clone(),
        created: network.created.clone(),
        labels: network.labels.clone(),
        ipam_config: Some(config),
        containers: Some(containers),
    };

    Json(Some(response))
}
