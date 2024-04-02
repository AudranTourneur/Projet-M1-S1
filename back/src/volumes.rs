use bollard::{service::Volume, Docker};
use lazy_static::lazy_static;
use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;
use std::{collections::HashMap, thread::current};
use std::sync::Mutex;
use bollard::volume::RemoveVolumeOptions;
use fs_extra::dir::{DirOptions, get_dir_content2};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

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
    pub current_folder: String,
    pub directories: Vec<String>,
    pub files: Vec<String>,
}


fn to_base64_url(data: &str) -> String {
    URL_SAFE.encode(data.as_bytes())
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

fn remove_prefix_from_path(path: &str, prefix: &str) -> String {
    if path.starts_with(prefix) {
        return path[prefix.len()..].to_string();
    }
    path.to_string()
}
 
#[get("/volume/<volume_name>/filesystem/<current_folder>")]
pub async fn volume_explorer_handler(volume_name: String, current_folder: Option<String>) -> Json<VolumeExplorerResponse>
{
    let docker = Docker::connect_with_local_defaults().unwrap();
    let volume = docker.inspect_volume(&volume_name).await.unwrap();
    println!("Volume: {:?}", volume);

    let root_folder = "/rootfs/var/lib/docker/volumes/";
    let initial_folder = format!("{}{}", root_folder, volume_name);
    println!("Initial folder: {}", initial_folder);

    let options = DirOptions::new();

    let full_path = format!("{}/{}", initial_folder, current_folder.clone().unwrap_or("".to_string()));

    let dir_content = get_dir_content2(full_path, &options);

    let dir_content = match dir_content {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading directory content");
            return Json(VolumeExplorerResponse {
                current_folder: "".to_string(),
                directories: vec![],
                files: vec![],
            });
        },
    };

    println!("aaaaaaaaaaa");

    let dir_folders = dir_content.directories;
    let dir_files = dir_content.files;

    println!("Folders: {:?}", dir_folders);
    println!("Files: {:?}", dir_files);

    println!("bbbbbbbbbb");

    let content = VolumeExplorerResponse {
        current_folder: current_folder.unwrap_or("".into()),
        directories: dir_folders,
        files: dir_files,
    };

    println!("cccccccccc");


    // for directory in dir_folders {
    //     println!("{}", directory); // print directory path
    // }
    // for file in dir_files {
    //     println!("{}", file); // print file path
    // }

    Json(content)

}