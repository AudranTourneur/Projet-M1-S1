use std::collections::HashMap;

use bollard::{secret::Network, Docker};
use futures::future::join_all;

use super::models::{NetworkContainerData, IpamConfigData, NetworkData};

pub fn get_ipam(network_response: Network) -> Vec<IpamConfigData> {
    let ipam_configs = network_response
        .clone()
        .ipam
        .unwrap()
        .clone()
        .config
        .unwrap();

    let config: Vec<IpamConfigData> = ipam_configs
        .iter()
        .map(|cfg| {
            let config = IpamConfigData {
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

pub fn get_containers(network_response: Network) -> HashMap<String, NetworkContainerData> {

    let network_containers = network_response
        .clone()
        .containers
        .unwrap()
        .clone();

    let containers: HashMap<String, NetworkContainerData> = network_containers
        .iter()
        .map(|(key, value)| {
            let container = NetworkContainerData {
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

pub async fn get_all_networks() -> Vec<NetworkData> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let base_networks = &docker.list_networks::<String>(None).await.unwrap();
        let my_networks = join_all(base_networks
        .iter()
        .map(|network| async {
            let docker: Docker = Docker::connect_with_local_defaults().unwrap();
            let network_response = docker.inspect_network::<String>(network.id.as_deref().unwrap_or(""), None).await.unwrap();
            let config = get_ipam(network_response.clone());
            let containers = get_containers(network_response);

            let our_network = NetworkData {
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