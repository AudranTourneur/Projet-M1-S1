use bollard::{service::Volume, Docker};
use lazy_static::lazy_static;
use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;
use std::{collections::HashMap, thread::current};
use std::sync::Mutex;
use bollard::volume::RemoveVolumeOptions;
use fs_extra::dir::{DirOptions, get_dir_content2};

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
pub struct VolumeResponse {
    pub volumes: Vec<VolumeData>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VolumeExplorerResponse {
    pub current_folder: Option<String>,
    pub directories: Vec<String>,
    pub files: Vec<String>,
}

// Define the static HashMap inside a lazy_static block
lazy_static! {
    static ref MUTABLE_MAP: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

// Function to update the static HashMap
fn update_static_map(key: String, value: u64) {
    // Lock the Mutex to perform mutable operations on the HashMap
    let mut map = MUTABLE_MAP.lock().unwrap();
    map.insert(key, value);
    // Mutex is automatically released when `map` goes out of scope
}

// Function to retrieve a value from the static HashMap
fn get_value_from_static_map(key: &str) -> Option<u64> {
    // Lock the Mutex to perform read operations on the HashMap
    let map = MUTABLE_MAP.lock().unwrap();
    map.get(key).cloned()
    // Mutex is automatically released when `map` goes out of scope
}

fn get_volume_size(vol: Volume) -> u64 {
    let name = vol.name.clone();
    let mountpoint = vol.mountpoint.clone();

    let maybe_size = get_value_from_static_map(&name);

    if maybe_size.is_some() {
        return maybe_size.unwrap();
    }

    println!("Calculating size for mountpoint {}", mountpoint);

    let size = fs_extra::dir::get_size(mountpoint.clone()).unwrap_or(0);

    println!("Size for mountpoint {} is {}", mountpoint, size);

    update_static_map(name, size);

    size
}

#[get("/volumes")]
pub async fn volumes_handler() -> Json<VolumeResponse> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let volumes = docker.list_volumes::<String>(None).await.unwrap();

    let mut volumes_data: Vec<VolumeData> = volumes
        .volumes
        .unwrap()
        .iter()
        .map(|volume| {
            let volume_data = VolumeData {
                name: volume.name.clone(),
                created_at: volume.created_at.clone().unwrap_or("UNDEFINED".to_string()),
                mountpoint: volume.mountpoint.clone(),
                size: get_volume_size(volume.clone()),
            };
            volume_data
        })
        .collect();

    volumes_data.sort_by(|a, b| a.name.cmp(&b.name));

    let response = VolumeResponse {
        volumes: volumes_data,
    };

    Json(response)
}

#[get("/volume/<name>")]
pub async fn volume_handler(name: String) -> Option<Json<VolumeData>> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    // Recherche du volume par son nom
    let volume = docker.inspect_volume(&name).await.ok()?;

    let volume_data = VolumeData {
        name: volume.name.clone(),
        created_at: volume.created_at.clone().unwrap_or("UNDEFINED".to_string()),
        mountpoint: volume.mountpoint.clone(),
        size: get_volume_size(volume),
    };
    
    Some(Json(volume_data))
}

#[post("/volume/<name>/remove")]
pub async fn delete_volume(name: &str) -> &'static str {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let options = Some(RemoveVolumeOptions {
        force: true,
        ..Default::default()
    });

    let _ = docker.remove_volume(name, options).await;

    "Volume deleted."
}
 
#[get("/volume/<volume_name>/filesystem/<current_folder>")]
pub async fn volume_explorer_handler(volume_name: String, current_folder: Option<String>) -> Json<VolumeExplorerResponse>
{
    let docker = Docker::connect_with_local_defaults().unwrap();
    let volume = docker.inspect_volume(&volume_name).await.unwrap();
    println!("Volume: {:?}", volume);

    let folder = "/rootfs/var/lib/docker/volumes/".to_string();
    let initial_folder = format!("{folder}{volume_name}", folder = folder, volume_name = volume_name);
    println!("Initial folder: {}", initial_folder);

    if current_folder.is_none() {
        let current_folder = initial_folder.clone();
    } else {
        let current_folder = format!("{folder}{current_folder}", folder = folder, current_folder = current_folder.clone().unwrap());
    }

    let mut options = DirOptions::new();
    let dir_content = get_dir_content2(current_folder.clone().unwrap(), &options);

    let dir_folder = dir_content.as_ref().unwrap().directories.clone();
    let dir_files = dir_content.as_ref().unwrap().files.clone();

    let content = VolumeExplorerResponse {
        current_folder: current_folder,
        directories: dir_folder,
        files: dir_files,
    };

    for directory in dir_content.as_ref().unwrap().directories.clone() {
        println!("{}", directory); // print directory path
    }
    for file in dir_content.unwrap().files {
        println!("{}", file); // print file path
    }

    Json(content)

}