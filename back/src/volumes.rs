
use std::collections::HashMap;
use lazy_static::lazy_static;
use bollard::{Docker, service::Volume};
use rocket::serde::{Serialize, Deserialize, json::Json};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct VolumeData {
    pub name: String,
    pub created_at: String,
    pub mountpoint: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeResponse {
    pub volumes: Vec<VolumeData>,
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

// fn get_volume_size(vol: Volume) -> u64 {
//     0
// }

fn get_volume_size(vol: Volume) -> u64 {
    let name = vol.name.clone();
    let mountpoint = vol.mountpoint.clone();
    
    let maybe_size = get_value_from_static_map(&name);

    if maybe_size.is_some() {
        return maybe_size.unwrap();
    }

    println!("Calculating size for mountpoint {}", mountpoint);

    let metadata = std::fs::metadata(mountpoint.clone());
        if metadata.is_err() {
        println!("Error getting metadata for mountpoint {}, error = {}", mountpoint.clone(), metadata.err().unwrap());
            return 0;
        }

    let size = metadata.unwrap().len();

    update_static_map(name, size);

    size
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
            size: get_volume_size(volume.clone()),
        };
        volume_data
    }).collect();

    volumes_data.sort_by(|a, b| a.name.cmp(&b.name));

    let response = VolumeResponse {
        volumes: volumes_data,
    };

    Json(response)
}
