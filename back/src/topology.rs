use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyContainer {
    pub name: String,
    pub image: String,
    pub icon_url: String,
    pub exposed_ports: Vec<u16>,
    pub exposed_volumes: Vec<String>,
    pub connected_to: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyPort {
    pub interface: String,
    pub number: u16,
    pub connected_to: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyVolume {
    pub name: String,
    pub size: u64,
    pub connected_to: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topology {
    pub containers: Vec<TopologyContainer>,
    pub ports: Vec<TopologyPort>,
    pub volumes: Vec<TopologyVolume>
}
