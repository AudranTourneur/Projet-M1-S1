use std::time::Duration;

use bollard::{
    container::{ListContainersOptions, StatsOptions},
    Docker,
};

use futures_util::stream::StreamExt;
use rocket::time::OffsetDateTime;

extern crate fs_extra;
use fs_extra::dir::get_size;
use tokio::time::sleep;

use crate::database::{self, get_volume_last_acquisition_timestamp};

pub async fn start_container_statistics_listeners() {
    loop {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let containers = &docker
            .list_containers::<String>(Some(ListContainersOptions::<String> {
                ..Default::default()
            }))
            .await
            .unwrap();

        for container in containers.iter() {
            //println!("Container data {:?}", container.unwrap().clone());
            get_container_statistics(container.id.clone().unwrap()).await;
        }

        sleep(Duration::from_secs(15));
    }
}

pub async fn start_volume_statistics_listeners() {
    loop {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let volumes = &docker
            .list_volumes::<String>(Default::default())
            .await
            .unwrap()
            .volumes
            .unwrap();

        for volume in volumes.iter() {
            //println!("{:?}", volume.clone());
            println!(
                "VOLUME NAME FOR GETTING SHOULD GET SIZE {}",
                should_get_volume_size(volume.name.clone()).await
            );
            /* if should_get_volume_size(volume.name.clone()).await {

                //get_volumes_size(volume.name.clone()).await;
            } */
        }

        sleep(Duration::from_secs(60 * 60));
    }
}

pub async fn get_container_statistics(container_id_to_get: String) {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let container = docker
        .inspect_container(&container_id_to_get, None)
        .await
        .unwrap();

    let container_id = container.id.as_ref().unwrap();

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
        }
        //else {
        //    println!(
        //        "Getting statistics for <Name={}, ID={}>",
        //        container_name, container_id
        //    );
        //}

        last_timestamp_acquisition = current_timestamp;

        // println!("cpu usage: {:?}", stats.cpu_stats.cpu_usage);

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

async fn should_get_volume_size(id_to_inspect: String) -> bool {
    // True if the last acquisition was more than 24 hours ago
    let time_threshold: u64 = 60 * 60 * 24;

    let current_timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;

    let vol_size = get_volume_last_acquisition_timestamp(id_to_inspect.clone()).await;

    let last_timestamp_acquisition = match vol_size {
        Ok(ts) => ts,
        Err(err) => {
            println!(
                "Error while doing database lookup for volume {}, doing nothing\nERROR: {:?}",
                id_to_inspect, err
            );
            return false;
        }
    };

    if (current_timestamp - last_timestamp_acquisition) < time_threshold {
        println!(
            "Volume {} was already updated in the last 24 hours",
            id_to_inspect
        );
        return false;
    }

    println!(
        "Volume {} was not updated in the last 24 hours",
        id_to_inspect
    );

    true
}

pub async fn get_volumes_size(id_to_inspect: String) {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let inspected = docker
        .inspect_container(&id_to_inspect, None)
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
            println!(
                "Updating volume size... <Name={}, Last Size={}> ",
                mountpoint_source, size
            );
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
    get_size(mountpoint_source.clone()).unwrap_or(0)
}
