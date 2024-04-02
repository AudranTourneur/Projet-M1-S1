use rocket::serde::{self, json::Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::containers::{get_all_containers, Container};


#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComposeData {
    pub name: String,
    pub containers: Vec<Container>,
}


#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComposeList {
    pub composes: Vec<ComposeData>,
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
            .find(|compose| compose.name == *name.clone());

        match compose {
            Some(compose) => {
                compose.containers.push(container.clone());
            }
            None => {
                let new_compose = ComposeData {
                    name: name.clone(),
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