use std::str::from_utf8;

use rocket::serde::json::Json;
use bollard::volume::RemoveVolumeOptions;
use fs_extra::dir::{DirOptions, get_dir_content2};

use crate::{docker::get_docker_socket, volumes::{common::remove_prefix_from_path, models::VolumeExplorerData}};

use super::{common::{get_volume_size, _from_base64_url}, models::{VolumeData, VolumeList}};

#[get("/volumes")]
pub async fn volumes_handler() -> Json<VolumeList> {
    let docker = get_docker_socket();

    let volumes = docker.list_volumes::<String>(None).await.unwrap();

    let volumes = volumes.volumes;
    let volumes = volumes.unwrap_or_default();

    let mut volumes_data: Vec<VolumeData> = volumes
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

    let response = VolumeList {
        volumes: volumes_data,
    };

    Json(response)
}

#[get("/volume/<name>")]
pub async fn volume_handler(name: String) -> Option<Json<VolumeData>> {
    let docker = get_docker_socket();

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
    let docker = get_docker_socket();

    let options = Some(RemoveVolumeOptions {
        force: true,
        ..Default::default()
    });

    let _ = docker.remove_volume(name, options).await;

    "Volume deleted."
}


 
#[get("/volume/<volume_name>/filesystem/<current_folder>")]
pub async fn volume_explorer_handler(volume_name: String, current_folder: Option<String>) -> Json<Option<VolumeExplorerData>>
{
    let docker = get_docker_socket();
    let volume = docker.inspect_volume(&volume_name).await;
    let volume = match volume {
        Ok(volume) => volume,
        Err(_) => {
            println!("Error reading volume");
            return Json(None);
        },
    };
    println!("Volume: {:?}", volume);

    let root_folder_without_slash = "/rootfs/var/lib/docker/volumes";
    let root_folder = format!("{}/", root_folder_without_slash);
    let initial_folder = format!("{}{}", root_folder, volume_name);
    println!("Initial folder: {}", initial_folder);
    
    //- 
    println!("Current folder: {:?}", current_folder.clone().unwrap_or("".to_string())); //Lw

    let decoded_u8 = _from_base64_url(&current_folder.clone().unwrap_or("".to_string()));
    let decoded = from_utf8(&decoded_u8).unwrap();
    println!("Decoded: {:?}", decoded);

    let full_path = format!("{}{}", initial_folder, decoded);

    println!("Full path: {}", full_path);

    let options = DirOptions {
        depth: 1,
    };
    let dir_content = get_dir_content2(full_path, &options);

    let dir_content = match dir_content {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading directory content");
            return Json(Some(VolumeExplorerData {
                current_folder: "".to_string(),
                directories: vec![],
                files: vec![],
            }));
        },
    };

    println!("aaaaaaaaaaa");

    
    // let prefix = format!("{}")

    let dir_folders = dir_content.directories.into_iter().map(|x| {
        remove_prefix_from_path(x, root_folder_without_slash)
    }).collect();
    let dir_files = dir_content.files.into_iter().map(|x| {
        remove_prefix_from_path(x, root_folder_without_slash)
    }).collect();

    println!("Folders: {:?}", dir_folders);
    println!("Files: {:?}", dir_files);

    println!("bbbbbbbbbb");

    let content = VolumeExplorerData {
        current_folder: decoded.to_string(),
        directories: dir_folders,
        files: dir_files,
    };

    println!("cccccccccc");

    Json(Some(content))

}