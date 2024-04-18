use std::time::Duration;

use bollard::{container::ListContainersOptions, Docker};

extern crate fs_extra;
use ::serde::Deserialize;
use fs_extra::dir::get_size;
use rocket::time::OffsetDateTime;
use tokio::time::sleep;

use crate::{
    database::{self, get_volume_last_acquisition_timestamp},
    volumes::common::get_all_volumes,
};

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

        sleep(Duration::from_secs(15)).await;
    }
}

pub async fn start_volume_statistics_listeners() {
    loop {
        let volumes = get_all_volumes().await;

        for volume in volumes.iter() {
            // let should_get_size = should_get_volume_size(volume.mountpoint.clone()).await;
            // println!(
            //     "VOLUME NAME FOR GETTING SHOULD GET SIZE {}",
            //     &should_get_size
            // );

            // if !should_get_size {
            //     continue;
            // }

            update_mountpoint_size(volume.mountpoint.clone()).await;
        }

        sleep(Duration::from_secs(60 * 60)).await;
        // sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Debug, Deserialize)]
struct DockerStatsOutput {
    #[serde(rename = "BlockIO")]
    block_io: String,
    #[serde(rename = "CPUPerc")]
    cpu_perc: String,
    // #[serde(rename = "Container")]
    // container: String,
    // #[serde(rename = "ID")]
    // id: String,
    // #[serde(rename = "MemPerc")]
    // mem_perc: String,
    #[serde(rename = "MemUsage")]
    mem_usage: String,
    // #[serde(rename = "Name")]
    // name: String,
    #[serde(rename = "NetIO")]
    net_io: String,
    // #[serde(rename = "PIDs")]
    // pids: String,
}

fn parse_memory_usage(original_memory_usage_input: String) -> u64 {
    // example of input and output
    // 1.2kB => 1_200
    // 1.2MB => 1_200_000
    // 1.2GB => 1_200_000_000
    // 1.2TB => 1_200_000_000_000

    let memory_usage_input = original_memory_usage_input.replace("iB", "");

    let memory_usage_input = memory_usage_input.replace("B", "");

    let memory_usage_input = memory_usage_input.replace(" ", "");

    let last_char = memory_usage_input.chars().last().unwrap();

    if last_char.is_numeric() {
        return memory_usage_input.parse::<u64>().unwrap_or(0);
    }

    let memory_usage_input_str = memory_usage_input.replace(last_char, "");

    let memory_usage_input = memory_usage_input_str.parse::<f64>();

    let memory_usage_input = match memory_usage_input {
        Ok(val) => val,
        Err(e) => {
            println!(
                " ====== Error while parsing memory usage: '{}' '{}' {:?}",
                original_memory_usage_input, memory_usage_input_str, e
            );
            0.0
        }
    };

    let memory_usage_input = match last_char {
        'k' => memory_usage_input * 1_000.0,
        'M' => memory_usage_input * 1_000_000.0,
        'G' => memory_usage_input * 1_000_000_000.0,
        'T' => memory_usage_input * 1_000_000_000_000.0,
        _ => memory_usage_input,
    };

    memory_usage_input as u64
}

pub async fn get_container_statistics(container_id: String) {
    // println!("Get container stats for {}", &container_id);
    let shell_command = format!(
        "docker container stats {} --format json --no-trunc --no-stream",
        &container_id
    );

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(shell_command)
        .output()
        .expect("failed to execute process");

    let output_string = String::from_utf8_lossy(&output.stdout).to_string();

    let stats_response: DockerStatsOutput = serde_json::from_str(&output_string).unwrap();

    // println!("Stats response {:?}", stats_response);

    let cpu_usage = stats_response
        .cpu_perc
        .replace('%', "")
        .parse::<f64>()
        .unwrap_or(0.0);

    // example of input string: 1.2GiB/1.95GiB
    // we should exttract 1.2GiB as an integer
    let memory_usage_str = stats_response.mem_usage.split('/').collect::<Vec<&str>>()[0];
    let memory_usage_parsed = parse_memory_usage(memory_usage_str.to_string()) as i32;

    let network_usage_received =
        parse_memory_usage(stats_response.net_io.split('/').collect::<Vec<&str>>()[0].to_string());
    let network_usage_sent =
        parse_memory_usage(stats_response.net_io.split('/').collect::<Vec<&str>>()[1].to_string());

    let io_usage_read = parse_memory_usage(
        stats_response.block_io.split('/').collect::<Vec<&str>>()[0].to_string(),
    );
    let io_usage_write = parse_memory_usage(
        stats_response.block_io.split('/').collect::<Vec<&str>>()[1].to_string(),
    );

    let current_timestamp = OffsetDateTime::now_utc().unix_timestamp();

    let stats = crate::models::ContainerStats {
        container_id: container_id.clone(),
        timestamp: current_timestamp as u64,
        cpu_usage,
        memory_usage: memory_usage_parsed,
        io_usage_read: io_usage_read as f64,
        io_usage_write: io_usage_write as f64,
        network_usage_up: network_usage_sent as f64,
        network_usage_down: network_usage_received as f64,
    };

    let _ = database::insert_container_stats(stats).await;
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
                "[stats.rs] Error while doing database lookup for volume {}, doing nothing\nERROR: {:?}",
                id_to_inspect, err
            );
            return false;
        }
    };

    if (current_timestamp - last_timestamp_acquisition) < time_threshold {
        println!(
            "[stats.rs] - Volume {} was already updated in the last 24 hours, timestamp {}",
            id_to_inspect, last_timestamp_acquisition
        );
        return false;
    }

    println!(
        "[stats.rs] - [OK] --- Volume {} was not updated in the last 24 hours, timestamp {}",
        id_to_inspect, last_timestamp_acquisition
    );

    true
}

pub async fn update_mountpoint_size(mountpoint: String) {
    println!("update_mountpoint_size for {}", mountpoint.clone());
    let size = get_mountpoint_size(mountpoint.clone()).await;
    let current_timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let volume_stats = crate::models::VolumeStats {
        path: mountpoint.clone(),
        volume_id: None,
        timestamp: current_timestamp as u64,
        disk_usage: size,
    };

    let res = database::insert_volume_stats(volume_stats).await;

    match res {
        Ok(_) => println!(
            "[stats.rs] - [OK] --- Volume {} updated with size {}",
            mountpoint, size
        ),
        Err(e) => println!(
            "[stats.rs] - [ERROR] --- Volume {} failed to update with size {} {:?}",
            mountpoint, size, e
        ),
    };
}

pub async fn get_mountpoint_size(mountpoint_source: String) -> u64 {
    let input = format!("/rootfs/{}", mountpoint_source.clone());
    let size = get_size(input);
    println!("size of {} is {:?}", mountpoint_source, size);

    let size = size.unwrap_or(0);

    size
}
