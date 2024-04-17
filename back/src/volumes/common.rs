use base64::{
    alphabet,
    engine::{
        self,
        general_purpose::{self},
    },
    Engine as _,
};
use bollard::service::Volume;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::docker::get_docker_socket;

use super::models::VolumeData;

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

pub fn get_volume_size(vol: Volume) -> u64 {
    let name = vol.name.clone();
    let mountpoint = vol.mountpoint.clone();

    let maybe_size = get_value_from_static_map(&name);

    // if maybe_size.is_some() {
    //     return maybe_size.unwrap();
    // }

    if let Some(maybe_size) = maybe_size {
        return maybe_size;
    }

    println!("Calculating size for mountpoint {}", mountpoint);

    let size = fs_extra::dir::get_size(mountpoint.clone()).unwrap_or(0);

    println!("Size for mountpoint {} is {}", mountpoint, size);

    update_static_map(name, size);

    size
}

pub fn remove_prefix_from_path(path: String, prefix: &str) -> String {
    if path.starts_with(prefix) {
        return path[prefix.len()..].to_string();
    }
    path.to_string()
}

pub fn _from_base64_url(data: &str) -> Vec<u8> {
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode(data)
        .unwrap()
}

pub async fn get_all_volumes() -> Vec<VolumeData> {
    let docker = get_docker_socket();

    // let containers = docker.list_containers::<String>(None).await.unwrap();

    // let new_containers:Vec<String> = containers.iter().map(|c| {
    //     if let Some(mounts) = c.mounts {
    //         for mount in mounts.iter() {
    //             if mount.driver.is_none() {
    //                 println!("Null driver volume {}", mount.source.unwrap_or("".into()));
    //             }
    //             "test".into()
    //         }
            
    //     }
    // }).collect();


    let volumes = docker.list_volumes::<String>(None).await.unwrap();

    let volumes = volumes.volumes;
    let volumes = volumes.unwrap_or_default();

    let mut volumes_data: Vec<VolumeData> = volumes
        .iter()
        .map(|volume| {
            VolumeData {
                name: volume.name.clone(),
                created_at: volume.created_at.clone().unwrap_or("UNDEFINED".to_string()),
                mountpoint: volume.mountpoint.clone(),
                size: get_volume_size(volume.clone()),
            }
        })
        .collect();

    volumes_data.sort_by(|a, b| a.name.cmp(&b.name));

    volumes_data
}
