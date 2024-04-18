use fs_extra::dir::{get_details_entry, get_dir_content2, DirEntryAttr, DirEntryValue, DirOptions};
use rocket::serde::json::Json;
use std::collections::HashSet;

use crate::{
    auth::JWT, utils::{self, from_base64_url}, volumes::{common::remove_prefix_from_path, models::VolumeExplorerData}
};

use super::common::get_all_volumes;
use super::models::{FileData, VolumeData, VolumeList, VolumeStatsResponse};

#[get("/volumes")]
pub async fn volumes_handler(_key: JWT) -> Json<VolumeList> {
    let volumes_data = get_all_volumes().await;

    let response = VolumeList {
        volumes: volumes_data,
    };

    Json(response)
}

#[get("/volume/<encoded_path>")]
pub async fn volume_handler(_key: JWT, encoded_path: String) -> Option<Json<VolumeData>> {
    let decoded = utils::from_base64_url(&encoded_path).unwrap();
    println!("decode {} => {}", encoded_path, decoded);

    let all_volumes = get_all_volumes().await;

    let volume = all_volumes.iter().find(|x| x.mountpoint == decoded);

    match volume {
        Some(volume) => Some(Json(volume.clone())),
        None => None,
    }
}

#[get("/volume/<encoded_volume_path>/filesystem/<encoded_current_folder>")]
pub async fn volume_explorer_handler(
    _key: JWT,
    encoded_volume_path: String,
    encoded_current_folder: Option<String>,
) -> Json<Option<VolumeExplorerData>> {

    let volume_path = from_base64_url(&encoded_volume_path);

    let volume_path = match volume_path {
        Some(volume_path) => volume_path,
        None => {
            println!(" ===============Error decoding volume path: {}", encoded_volume_path.clone());
            return Json(None);
        }
    };

    let mountpoint = volume_path;

    let root_folder_without_slash = format!("/rootfs{}", mountpoint.clone());

    let initial_folder = format!("{}/", &mountpoint);

    println!("Initial folder: {}", initial_folder);

    println!(
        "Encoded current folder: {:?}",
        encoded_current_folder.clone().unwrap_or("".to_string())
    );

    let decoded = from_base64_url(&encoded_current_folder.clone().unwrap_or("".to_string()));

    let decoded = match decoded {
        Some(decoded) => decoded,
        None => {
            println!(" =========== Error decoding current folder {}", encoded_current_folder.clone().unwrap_or("".to_string()));
            return Json(None);
        }
    };

    println!("Decoded: {:?}", decoded);

    let full_path = format!("{}{}", root_folder_without_slash, decoded);

    println!("Full path: {}", full_path);

    let options = DirOptions { depth: 1 };
    let dir_content = get_dir_content2(full_path.clone(), &options);

    let dir_content = match dir_content {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading directory content: {}", full_path);
            return Json(Some(VolumeExplorerData {
                current_folder: "".to_string(),
                directories: vec![],
                files: vec![],
            }));
        }
    };

    let dir_folders: Vec<String> = dir_content
        .directories
        .into_iter()
        .map(|x| remove_prefix_from_path(x, &root_folder_without_slash))
        .collect();
    let dir_files: Vec<String> = dir_content
        .files
        .into_iter()
        .map(|x| remove_prefix_from_path(x, &root_folder_without_slash))
        .collect();

    println!("Folders: {:?}", dir_folders.clone());
    println!("Files: {:?}", dir_files.clone());

    let dir_folders = details_fdir(dir_folders, full_path.clone(), &root_folder_without_slash);
    let dir_files = details_fdir(dir_files, full_path, &root_folder_without_slash);

    // remove the first folder
    let dir_folders = dir_folders.into_iter().skip(1).collect();

    let content = VolumeExplorerData {
        current_folder: decoded.to_string(),
        directories: dir_folders,
        files: dir_files,
    };

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

fn details_fdir(
    dir_folders: Vec<String>,
    full_path: String,
    root_folder_without_slash: &str,
) -> Vec<FileData> {
    let folder_data: Vec<Option<FileData>> = dir_folders
        .iter()
        .map(|x| {
            println!("full path {}", full_path.clone());
            let tmp = format!("{}{}", root_folder_without_slash, x.clone());
            println!("{}", tmp);

            let mut config = HashSet::new();
            config.insert(DirEntryAttr::Name);
            config.insert(DirEntryAttr::Size);
            config.insert(DirEntryAttr::BaseInfo);

            let res = get_details_entry(tmp.clone(), &config);

            let res = match res {
                Ok(res) => res,
                Err(e) => {
                    println!("Unlucky :c | {:?} | attepting to read {}", e, tmp.clone());
                    return None;
                }
            };

            let name = res.get(&DirEntryAttr::Name).unwrap();
            let name = extract_value(name);

            let size = res.get(&DirEntryAttr::Size).unwrap();
            let size = extract_value(size);

            let file_data = FileData { name, size };

            Some(file_data)
        })
        .collect();

    folder_data.into_iter().flatten().collect()
}

#[get("/statistics-historical/volume/<encoded_path>")]
pub async fn volume_stats_handler(_key: JWT, encoded_path: &str) -> Json<VolumeStatsResponse> {
    let path = from_base64_url(encoded_path).unwrap_or("".to_string());
    println!("################################ Decoded path for statistics historical volume encoded path etc etc etc: {}", path.clone());
    let db_res = crate::database::get_historical_statistics_for_volume(path).await;

    match db_res {
        Ok(stats) => {
            println!("Volume statistics: {:?}", stats);
            let res = VolumeStatsResponse { stats };

            Json(res)
        }
        Err(e) => {
            println!("Error getting volume statistics: {}", e);

            let res = VolumeStatsResponse { stats: vec![] };

            Json(res)
        }
    }
}
