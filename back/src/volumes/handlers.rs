use std::str::from_utf8;
use std::collections::HashSet;
use rocket::serde::json::Json;
use bollard::volume::RemoveVolumeOptions;
use fs_extra::dir::{DirOptions, DirEntryAttr, DirEntryValue, get_details_entry, get_dir_content2};

use crate::{docker::get_docker_socket, volumes::{common::remove_prefix_from_path, models::VolumeExplorerData}};

use super::common::get_all_volumes;
use super::{common::{get_volume_size, _from_base64_url}, models::{VolumeData, VolumeList, FileData}};

#[get("/volumes")]
pub async fn volumes_handler() -> Json<VolumeList> {
    let volumes_data = get_all_volumes().await;

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
    
    println!("Current folder: {:?}", current_folder.clone().unwrap_or("".to_string()));

    let decoded_u8 = _from_base64_url(&current_folder.clone().unwrap_or("".to_string()));
    let decoded = from_utf8(&decoded_u8).unwrap();
    println!("Decoded: {:?}", decoded);

    let full_path = format!("{}{}", initial_folder, decoded);

    println!("Full path: {}", full_path);

    let options = DirOptions {
        depth: 1,
    };
    let dir_content = get_dir_content2(full_path.clone(), &options);

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

    let dir_folders : Vec<String> = dir_content.directories.into_iter().map(|x| {
        remove_prefix_from_path(x, root_folder_without_slash)
    }).collect();
    let dir_files : Vec<String> =  dir_content.files.into_iter().map(|x| {
        remove_prefix_from_path(x, root_folder_without_slash)
    }).collect();

    println!("Folders: {:?}", dir_folders.clone());
    println!("Files: {:?}", dir_files.clone());

    let dir_folders = details_fdir(dir_folders, full_path.clone(), root_folder_without_slash);
    let dir_files = details_fdir(dir_files, full_path, root_folder_without_slash);

    println!("bbbbbbbbbb");

    let content = VolumeExplorerData {
        current_folder: decoded.to_string(),
        directories: dir_folders,
        files: dir_files,
    };

    println!("cccccccccc");

    Json(Some(content))

}


fn extract_value(val: &DirEntryValue) -> String {
    match val {
        DirEntryValue::String(s) => s.into(),
        DirEntryValue::Boolean(b) => b.to_string(),
        DirEntryValue::SystemTime(_) => "...".into(),
        DirEntryValue::U64(u) => u.to_string(),
    }
}

fn details_fdir(dir_folders : Vec<String>, full_path : String, root_folder_without_slash : &str) -> Vec<FileData> {

    let folder_data : Vec<Option<FileData>> = dir_folders.iter().map(|x| {
        println!("full path {}",full_path.clone());
        let tmp = format!("{}{}", root_folder_without_slash.clone(), x.clone());
        println!("{}",tmp);

        let mut config = HashSet::new();
            config.insert(DirEntryAttr::Name);
            config.insert(DirEntryAttr::Size);
            config.insert(DirEntryAttr::BaseInfo);
        
        let res = get_details_entry(tmp.clone(), &config);

        let res = match res {
            Ok(res) =>  res,
            Err(e) => {
                println!("Unlucky :c | {:?} | attepting to read {}", e, tmp.clone());
                return None;
            }
        };

        let name = res.get(&DirEntryAttr::Name).unwrap();
        let name = extract_value(name);
        println!("NAME = {}", name);

        let size = res.get(&DirEntryAttr::Size).unwrap();
        let size = extract_value(size);
        println!("SIZE = {}", size);

        let file_data = FileData {
            name : name,
            size : size,
        };

        Some(file_data)

    }).collect();

    let res = folder_data.into_iter().filter_map(|e|e).collect();
    
    res
}