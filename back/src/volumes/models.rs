use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeData {
    pub name: String,
    pub created_at: String,
    pub mountpoint: String,
    pub size: u64,
    pub is_mountpoint: bool,
    pub base64_name: String,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeList {
    pub volumes: Vec<VolumeData>,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeExplorerData {
    pub current_folder: String,
    //pub directories : Vec<String>,
    //pub files : Vec<String>,
    pub directories: Vec<FileData>,
    pub files: Vec<FileData>,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct FileData {
    pub name: String,
    pub size: String,
    //pub name64 : String,
    /* pub creation_time : String,
    pub permissions : String,
    pub owner : String,
    pub group : String,
    pub isDirectory : String, */
}

#[derive(Serialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeStatsResponse {
    pub stats : Vec<crate::database::VolumeRow>,
}