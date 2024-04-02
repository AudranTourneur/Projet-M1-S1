use rocket::serde::{self};
use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;
use crate::containers::{get_all_containers, Container};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};


#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComposeData {
    pub file_path: String,
    pub id: String,
    pub containers: Vec<Container>,
}


#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComposeList {
    pub composes: Vec<ComposeData>,
}

fn to_base64_url(data: &str) -> String {
    URL_SAFE.encode(data.as_bytes())
}

pub async fn get_all_composes() -> ComposeList {
    let mut compose_data = ComposeList {
        composes: vec![],
    };

    let listed_containers: Vec<Container> = get_all_containers().await;

    for container in listed_containers.iter() {
        let name = container.labels.as_ref().unwrap().get("com.docker.compose.project.config_files");
        let name = match name {
            Some(name) => name,
            None => continue,
        };

        let compose = compose_data
            .composes
            .iter_mut()
            .find(|compose| compose.file_path == *name.clone());

        match compose {
            Some(compose) => {
                compose.containers.push(container.clone());
            }
            None => {
                let new_compose = ComposeData {
                    file_path: name.clone(),
                    id: to_base64_url(name),
                    containers: vec![container.clone()],
                };
                compose_data.composes.push(new_compose);
            }
        };
    }

    compose_data
}

#[get("/composes")]
pub async fn composes_handler() -> Json<ComposeList> {
    let listed_composes = get_all_composes().await;

    Json(listed_composes)
}

#[get("/composes/<id>")]
pub async fn compose_handler(id: String) -> Json<ComposeData> {
    let listed_composes = get_all_composes().await;

    let compose = listed_composes
        .composes
        .iter()
        .find(|compose| compose.id == id)
        .unwrap();

    Json(compose.clone())
}
