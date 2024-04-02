use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeData {
    pub name: String,
    pub created_at: String,
    pub mountpoint: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeList {
    pub volumes: Vec<VolumeData>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeExplorerData {
    pub current_folder: String,
    pub directories: Vec<String>,
    pub files: Vec<String>,
}