use rocket::serde::json::Json;
use bollard::volume::RemoveVolumeOptions;
use fs_extra::dir::{DirOptions, get_dir_content2};

use crate::{docker::get_docker_socket, volumes::models::VolumeExplorerData};

use super::{common::get_volume_size, models::{VolumeData, VolumeList}};

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
            return Json(Some(VolumeExplorerData {
                current_folder: "".to_string(),
                directories: vec![],
                files: vec![],
            }));
        },
    };

    println!("aaaaaaaaaaa");

    let dir_folders = dir_content.directories;
    let dir_files = dir_content.files;

    println!("Folders: {:?}", dir_folders);
    println!("Files: {:?}", dir_files);

    println!("bbbbbbbbbb");

    let content = VolumeExplorerData {
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

    Json(Some(content))

}