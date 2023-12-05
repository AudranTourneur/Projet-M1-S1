use bollard::Docker;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct VolumeData {
    pub name: String,
    pub created_at: String,
    pub mountpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeResponse {
    pub volumes: Vec<VolumeData>,
}

#[get("/volumes")]
pub async fn volumes_handler() -> Json<VolumeResponse> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let volumes = docker.list_volumes::<String>(None).await.unwrap();

    let mut volumes_data: Vec<VolumeData> = volumes.volumes.unwrap().iter().map(|volume| {
        let volume_data = VolumeData {
            name: volume.name.clone(),
            created_at: volume.created_at.clone().unwrap_or("UNDEFINED".to_string()),
            mountpoint: volume.mountpoint.clone(),
        };
        volume_data
    }).collect();

    volumes_data.sort_by(|a, b| a.name.cmp(&b.name));

    let response = VolumeResponse {
        volumes: volumes_data,
    };

    Json(response)
}
