use std::process::Command;

use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use crate::containers::{common::get_all_containers, models::ContainerData};

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComposeData {
    pub file_path: String,
    pub file_content: String,
    pub id: String,
    pub containers: Vec<ContainerData>,
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
    let mut compose_data = ComposeList { composes: vec![] };

    let listed_containers: Vec<ContainerData> = get_all_containers().await;

    for container in listed_containers.iter() {
        let labels = container.labels.as_ref();
        let labels = match labels {
            Some(labels) => labels,
            None => continue,
        };
        let name = labels.get("com.docker.compose.project.config_files");
        let name = match name {
            Some(name) => name,
            None => continue,
        };

        let compose = compose_data
            .composes
            .iter_mut()
            .find(|compose| compose.file_path == *name.clone());

        let file_content = std::fs::read_to_string(format!("/rootfs/{}", name.clone()));

        let file_content = match file_content {
            Ok(file_content) => file_content,
            Err(_) => continue,
        };

        match compose {
            Some(compose) => {
                compose.containers.push(container.clone());
            }
            None => {
                let new_compose = ComposeData {
                    file_path: name.clone(),
                    id: to_base64_url(name),
                    containers: vec![container.clone()],
                    file_content,
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
pub async fn compose_handler(id: String) -> Json<Option<ComposeData>> {
    let listed_composes = get_all_composes().await;

    let compose = listed_composes
        .composes
        .iter()
        .find(|compose| compose.id == id);
    let compose = match compose {
        Some(compose) => compose,
        None => return Json(None),
    };

    Json(Some(compose.clone()))
}

fn from_base64_url(data: &str) -> String {
    let vec: Vec<u8> = URL_SAFE.decode(data).unwrap();
    String::from_utf8(vec).unwrap()
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ShellOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: String,
}

#[get("/composes/<id>/start")]
pub async fn compose_start_handler(id: &str) -> Json<ShellOutput> {
    let id = from_base64_url(id);
    println!("Attemting to Docker Compose UP: {}", id);
    let output = Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(format!("/rootfs{}", id))
        .arg("up")
        .arg("-d")
        .output()
        .expect("failed to execute process");

    let shell_output = ShellOutput {
        stdout: std::str::from_utf8(&output.stdout).unwrap_or("").into(),
        stderr: std::str::from_utf8(&output.stderr).unwrap_or("").into(),
        status: output.status.to_string(),
    };

    println!("docker-compose up -d {:?}", shell_output);

    Json(shell_output)
}

#[get("/composes/<id>/stop")]
pub async fn compose_stop_handler(id: &str) -> Json<bool> {
    let id = from_base64_url(id);
    let output = Command::new("docker")
        .arg("compose")
        .arg("down")
        .arg("-f")
        .arg(format!("/rootfs/{}", id))
        .output()
        .expect("failed to execute process");
    let hello = output.stdout;
    let hello = std::str::from_utf8(&hello).unwrap();

    println!("docker-compose up -d {}", hello);

    Json(true)
}
