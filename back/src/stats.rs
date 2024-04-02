use bollard::{
    container::{ListContainersOptions, StatsOptions},
    Docker,
};

use futures_util::stream::StreamExt;
use rocket::time::OffsetDateTime;

extern crate fs_extra;
use fs_extra::dir::get_size;

use crate::database;

pub async fn start_statistics_listeners() {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let containers = &docker
        .list_containers::<String>(Some(ListContainersOptions::<String> {
            ..Default::default()
        }))
        .await
        .unwrap();

    for container in containers.iter() {
        // println!("aaaa id = {}", container.id.clone().unwrap());
        rocket::tokio::spawn(get_container_statistics(container.id.clone().unwrap()));
        // rocket::tokio::spawn(get_volumes_size(container.id.clone().unwrap()));
    }
}

pub async fn get_container_statistics(container_id_to_get: String) {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let container = docker
        .inspect_container(&container_id_to_get, None)
        .await
        .unwrap();

    let container_id = container.id.as_ref().unwrap();
    let container_name = container.name.unwrap_or("/".to_owned());

    // println!("container_id: {}", container_id);

    //let stats = &docker.stats("docker-postgres", Some(bollard::container::StatsOptions { stream: true, ..Default::default() })).try_collect::<Vec<_>>().await.unwrap();

    let stream = &mut docker.stats(
        &container_id,
        Some(StatsOptions {
            stream: true,
            ..Default::default()
        }),
    );

    let mut last_timestamp_acquisition = 0;

    let time_threshold = 15;

    while let Some(Ok(stats)) = stream.next().await {
        let current_timestamp = OffsetDateTime::now_utc().unix_timestamp();

        let diff = current_timestamp - last_timestamp_acquisition;

        if diff < time_threshold {
            continue;
        } else {
            println!("Getting statistics for <Name={}, ID={}>", container_name, container_id);
        }

        last_timestamp_acquisition = current_timestamp;

        println!("cpu usage: {:?}", stats.cpu_stats.cpu_usage);

    let stats = crate::models::ContainerStats {
    
            container_id: container_id.clone(),
            timestamp: current_timestamp as u64,
            cpu_usage: stats.cpu_stats.cpu_usage.total_usage as f64,
            memory_usage: stats.memory_stats.usage.unwrap_or_default() as i32,
            io_usage_read: 0.0,
            io_usage_write: 0.0,
            network_usage_up: 0.0,
            network_usage_down: 0.0,
        };

        let _ = database::insert_container_stats(stats).await;
    }
}

pub async fn get_volumes_size(id_to_inspect : String){
    let docker = Docker::connect_with_local_defaults().unwrap();

    let inspected = docker.inspect_container(&id_to_inspect, None)
    .await
    .unwrap();

    let mountpoint_info = inspected.mounts.clone().unwrap();
    let source = mountpoint_info[0].source.clone().unwrap();

    let folder = "/rootfs/".to_string();
    let mountpoint_source = format!("{folder}{source}", folder = folder, source = source);     


    let mut last_timestamp_acquisition = 0;
    let time_threshold = 60 * 60;
    let mut size = get_mountpoint_size(mountpoint_source.clone()).await;


    loop {
        let current_timestamp = OffsetDateTime::now_utc().unix_timestamp();

        let diff = current_timestamp - last_timestamp_acquisition;

        if diff < time_threshold {
            continue;
        } else {
            println!("Updating volume size... <Name={}, Last Size={}> ", mountpoint_source, size);
        }

        last_timestamp_acquisition = current_timestamp;

        size = get_mountpoint_size(mountpoint_source.clone()).await;
        let volume_stats = crate::models::VolumeStats {
            volume_id: id_to_inspect.clone(),
            timestamp: current_timestamp as u64,
            disk_usage: size as i32,
        };

        let _ = database::insert_volume_stats(volume_stats).await;
    }
}

pub async fn get_mountpoint_size(mountpoint_source: String) -> u64 {
    let size = get_size(mountpoint_source.clone()).unwrap_or(0);
    size
}