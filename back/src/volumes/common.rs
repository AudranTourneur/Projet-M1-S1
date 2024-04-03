
use bollard::service::Volume;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use base64::{alphabet, decode, engine::{self, general_purpose::{self, URL_SAFE}}, Engine as _};

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

    if maybe_size.is_some() {
        return maybe_size.unwrap();
    }

    println!("Calculating size for mountpoint {}", mountpoint);

    let size = fs_extra::dir::get_size(mountpoint.clone()).unwrap_or(0);

    println!("Size for mountpoint {} is {}", mountpoint, size);

    update_static_map(name, size);

    size
}

fn _remove_prefix_from_path(path: &str, prefix: &str) -> String {
    if path.starts_with(prefix) {
        return path[prefix.len()..].to_string();
    }
    path.to_string()
}


fn _to_base64_url(data: &str) -> String {
    URL_SAFE.encode(data.as_bytes())
}

pub fn _from_base64_url(data: &str) -> Vec<u8> {
    engine::GeneralPurpose::new(
        &alphabet::URL_SAFE,
        general_purpose::NO_PAD)
.decode(data).unwrap()
}