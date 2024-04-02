use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct IpamConfigData {
    pub subnet: String,
    pub ip_range: String,
    pub gateway: String,
    pub aux_addresses: Option<HashMap<String, String, RandomState>>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct NetworkContainerData {
    pub name: String,
    pub endpoint_id: String,
    pub mac_address: String,
    pub ipv4_address: String,
    pub ipv6_address: String,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct NetworkData {
    pub id: String,
    pub name: String,
    pub created: String,
    pub labels: Option<HashMap<String, String, RandomState>>,
    pub ipam_config: Option<Vec<IpamConfigData>>,
    pub containers: Option<HashMap<String, NetworkContainerData>>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct NetworkList {
    pub networks: Vec<NetworkData>,
}