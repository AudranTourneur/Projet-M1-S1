use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct HistoryResponse {
    pub id: String,
    pub created: i64,
    pub created_by: String,
    pub tags: Vec<String>,
    pub size: i64,
    pub comment: String,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ImageData {
    pub id: String,
    pub tags: Vec<String>,
    pub size: i64,
    pub created: i64,
    pub history: Option<Vec<HistoryResponse>>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ImageList {
    pub images: Vec<ImageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImagePullRequest {
    pub id: String,
}

