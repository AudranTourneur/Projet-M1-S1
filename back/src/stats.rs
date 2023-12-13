use bollard::{
    container::{ListContainersOptions, StatsOptions},
    Docker,
};

use futures_util::stream::StreamExt;
use rocket::time::{OffsetDateTime, PrimitiveDateTime};

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
        println!("aaaa id = {}", container.id.clone().unwrap());
        rocket::tokio::spawn(get_container_statistics(container.id.clone().unwrap()));
    }
}

pub async fn get_container_statistics(container_id_to_get: String) {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let container = docker
        .inspect_container(&container_id_to_get, None)
        .await
        .unwrap();

    let container_id = container.id.as_ref().unwrap();

    println!("container_id: {}", container_id);

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
            println!("STATS FOR {}", container_id);
        }

        last_timestamp_acquisition = current_timestamp;

        let now_odt = OffsetDateTime::now_utc();
        let now_pdt = PrimitiveDateTime::new(now_odt.date(), now_odt.time());

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
