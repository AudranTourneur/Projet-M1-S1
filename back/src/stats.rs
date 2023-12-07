use bollard::{Docker, container::StatsOptions};

use futures_util::stream::StreamExt;

pub async fn start_statistics_listeners() {
        get_container_statistics("docker-postgres".to_string()).await;
}

pub async fn get_container_statistics(container_id_to_get: String) {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let container = docker.inspect_container(&container_id_to_get, None).await.unwrap();

    let container_id = container.id.as_ref().unwrap();

    println!("container_id: {}", container_id);

    //let stats = &docker.stats("docker-postgres", Some(bollard::container::StatsOptions { stream: true, ..Default::default() })).try_collect::<Vec<_>>().await.unwrap();
    
    let stream = &mut docker.stats(&container_id, Some(StatsOptions {
       stream: true,
       ..Default::default()
    }));

     while let Some(Ok(stats)) = stream.next().await {
                    println!(
                        "plop {} - {:?}: {:?} {:?}",
                        container_id, &container.name, container.image, stats.memory_stats.usage
                    );
                }

    println!("end");
}
