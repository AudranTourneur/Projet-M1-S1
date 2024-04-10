use std::collections::HashMap;

use bollard::{secret::Network, Docker};
use futures::future::join_all;

use crate::docker::get_docker_socket;

use super::models::{IpamConfigData, NetworkContainerData, NetworkData};

pub fn get_ipam(network_response: Network) -> Vec<IpamConfigData> {
    let ipam = network_response.ipam.clone();
    let ipam = match ipam {
        Some(ipam) => ipam,
        None => return Vec::new(),
    };

    let ipam_configs = ipam.clone().config;
    let ipam_configs = match ipam_configs {
        Some(ipam_configs) => ipam_configs,
        None => return Vec::new(),
    };

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
    let network_containers = network_response.clone().containers;
    let network_containers = match network_containers {
        Some(network_containers) => network_containers,
        None => return HashMap::new(),
    };

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
    let docker: Docker = get_docker_socket();
    let base_networks = &docker.list_networks::<String>(None).await;
    let base_networks = match base_networks {
        Ok(networks) => networks,
        Err(_) => return vec![],
    };

    let my_networks = join_all(base_networks.iter().map(|network| async {
        let docker: Docker = get_docker_socket();
        let network_response = docker
            .inspect_network::<String>(network.id.as_deref().unwrap_or(""), None)
            .await;
        let network_response = match network_response {
            Ok(network_response) => network_response,
            Err(_) => {
                return NetworkData {
                    id: "UNDEFINED".to_string(),
                    name: "UNDEFINED".to_string(),
                    created: "UNDEFINED".to_string(),
                    labels: None,
                    ipam_config: None,
                    containers: None,
                }
            }
        };
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
    }));

    my_networks.await
}
