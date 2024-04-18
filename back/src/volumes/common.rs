use futures::future::join_all;

use crate::{database::get_volume_latest_size, docker::get_docker_socket, utils};

use super::models::VolumeData;

pub fn remove_prefix_from_path(path: String, prefix: &str) -> String {
    if path.starts_with(prefix) {
        return path[prefix.len()..].to_string();
    }
    path.to_string()
}

pub async fn get_all_volumes() -> Vec<VolumeData> {
    let docker = get_docker_socket();

    let volumes = docker.list_volumes::<String>(None).await.unwrap();

    let volumes = volumes.volumes;

    let volumes = volumes.unwrap_or_default();

    let volumes_data: Vec<_> = volumes
        .iter()
        .map(|volume| async {
            VolumeData {
                name: volume.name.clone(),
                created_at: volume.created_at.clone().unwrap_or("UNDEFINED".to_string()),
                mountpoint: volume.mountpoint.clone(),
                size: get_volume_latest_size(volume.mountpoint.clone()).await,
                is_mountpoint: false,
                base64_name: utils::to_base64_url(&volume.mountpoint),
            }
        })
        .collect();

    let mut volumes_data = join_all(volumes_data).await;

    let containers = docker.list_containers::<String>(None).await.unwrap();

    for container in containers.iter() {
        if let Some(mounts) = &container.mounts {
            for mount in mounts.iter() {
                if mount.driver.is_none() {
                    println!(
                        "Null driver volume {}",
                        &mount.clone().source.unwrap_or("".into())
                    );
                    if mount.clone().destination.is_some() {
                        let mountpoint = mount.clone().source.unwrap_or("".into());
                        volumes_data.push(VolumeData {
                            name: mountpoint.clone(),
                            created_at: "UNDEFINED".to_string(),
                            mountpoint: mountpoint.clone(),
                            size: get_volume_latest_size(mountpoint.clone())
                            .await,
                            is_mountpoint: true,
                            base64_name: utils::to_base64_url(&mountpoint),
                        });
                    }
                }
            }
        }
    }

    volumes_data.sort_by(|a, b| a.size.cmp(&b.size));
    volumes_data.reverse();

    volumes_data
}
