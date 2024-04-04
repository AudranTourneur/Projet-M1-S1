use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone)]
pub enum OurPortTypeEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PortData {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    #[serde(rename = "type")]
    pub typ: Option<OurPortTypeEnum>,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]

pub struct ContainerData {
    pub icon_url: Option<String>,
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    // pub network: String,
    pub networks: Vec<String>,
    pub volumes: Vec<String>,
    pub status: String,
    pub ports: Vec<PortData>,
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub compose_file: Option<String>,
    pub raw_data: Option<String>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerList {
    pub containers: Vec<ContainerData>,
}

#[derive(Serialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerStatsResponse {
    pub stats: Vec<crate::database::ContainerStatisticsRow>,
}


#[derive(Serialize, Debug, TS, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerPortRebindRequest {
    pub ports: Vec<ContainerPortRebind>
}


#[derive(Serialize, Debug, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerPortRebind {
    pub host: u16,
    pub internal: u16
}