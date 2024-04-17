use std::env;

use std::error::Error;

use crate::models::ContainerStats;
use crate::models::VolumeStats;

use clickhouse::Row;

pub async fn insert_container_stats(
    container_statistics: ContainerStats,
) -> Result<(), Box<dyn Error>> {
    let clickhouse_client = get_clickhouse_client();

    let insert_query = format!(
        "INSERT INTO container_statistics (id, timestamp, cpu_usage, memory_usage, io_usage_read, io_usage_write, network_usage_up, network_usage_down) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
        container_statistics.container_id,
        container_statistics.timestamp,
        container_statistics.cpu_usage,
        container_statistics.memory_usage,
        container_statistics.io_usage_read,
        container_statistics.io_usage_write,
        container_statistics.network_usage_up,
        container_statistics.network_usage_down,
    );

    // todo: replace by prepared statement!!!
    let insert = clickhouse_client.query(&insert_query);

    let res = insert.execute().await;

    match res {
        Ok(_) => (),
        Err(e) => println!(
            "Error inserting container statistics: {} | REQ = {}",
            e, insert_query
        ),
    };

    Ok(())
}

use clickhouse::Client;

pub fn get_clickhouse_client() -> Client {
    Client::default()
        // .with_url("http://okidocky-db:8123")
        .with_url(format!("http://127.0.0.1:{}", env::var("PORT_CLICKHOUSE").unwrap()).as_str())
        .with_user("root")
        .with_password(env::var("CLICKHOUSE_PASSWORD").unwrap())
        .with_database("okidocky")
}

pub async fn init_clickhouse_database() -> Result<(), Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();

    let init_query_containers = include_str!("../resources/clickhouse_init_containers.sql");

    let res = client.query(init_query_containers).execute().await;

    match res {
        Ok(_) => println!("Clickhouse database initialized"),
        Err(e) => {
            println!("WARNING: Error initializing clickhouse database: {}", e);
            return Err(Box::new(e));
        }
    };

    let init_query_volumes = include_str!("../resources/clickhouse_init_volumes.sql");

    let res = client.query(init_query_volumes).execute().await;

    match res {
        Ok(_) => println!("Clickhouse database initialized"),
        Err(e) => {
            println!("Error initializing clickhouse database: {}", e);
            return Err(Box::new(e));
        }
    };

    Ok(())
}

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Row, Deserialize, Serialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ContainerStatisticsRow {
    ts: u32,
    mem: u64,
    cpu: f32,
    io_read: f32,
    io_write: f32,
    net_up: f32,
    net_down: f32,
}

pub async fn get_historical_statistics_for_container(
    id: String,
) -> Result<Vec<ContainerStatisticsRow>, Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();

    let mut cursor = client
    .query("SELECT timestamp AS ts, memory_usage AS mem, cpu_usage AS cpu, io_usage_read AS io_read, io_usage_write AS io_write, network_usage_up AS net_up, network_usage_down AS net_down FROM container_statistics WHERE id = ? ORDER BY timestamp ASC")
    .bind(id)
    .fetch::<ContainerStatisticsRow>()?;

    let mut vector_response: Vec<ContainerStatisticsRow> = vec![];

    while let Some(row) = cursor.next().await? {
        vector_response.push(row)
    }

    Ok(vector_response)
}

#[derive(Row, Deserialize)]
struct VolumeLastAcquisitionResponseRow {
    max_timestamp: u64,
}

pub async fn get_volume_last_acquisition_timestamp(id: String) -> Result<u64, Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();

    let cursor = client
        .query("SELECT max(timestamp) AS max_timestamp FROM volume_statistics WHERE id = ?")
        .bind(id.clone())
        .fetch::<VolumeLastAcquisitionResponseRow>();

    match cursor {
        Ok(mut cursor) => {
            println!("Cursor created successfully");

            let mut vector_response: Vec<VolumeLastAcquisitionResponseRow> = vec![];

            while let Some(row) = cursor.next().await? {
                vector_response.push(row)
            }

            if vector_response.len() == 0 {
                return Ok(0);
            }

            return Ok(vector_response[0].max_timestamp);
        }
        Err(e) => {
            match e {
                clickhouse::error::Error::NotEnoughData => {
                    return Ok(0);
                }
                _ => {
                    println!(
                "Error while doing database lookup for volume {}, doing nothing\nERROR: {:?}",
                id.clone(), e
            );
                    return Err(Box::new(e));
                }
            };
        }
    };
}

pub async fn insert_volume_stats(volume_statistics: VolumeStats) -> Result<(), Box<dyn Error>> {
    let clickhouse_client = get_clickhouse_client();

    let insert_query = format!(
        "INSERT INTO volume_statistics (id, timestamp, disk_usage)
        VALUES ('{}', '{}', '{}')",
        volume_statistics.volume_id, volume_statistics.timestamp, volume_statistics.disk_usage,
    );

    let insert = clickhouse_client.query(&insert_query);

    let res = insert.execute().await;

    match res {
        Ok(_) => (),
        Err(e) => println!("Error inserting volume statistics: {}", e),
    };

    Ok(())
}

#[derive(Row, Deserialize, Serialize, Debug, TS)]
pub struct VolumeRow {
    ts: u32,
    dsk: u64,
}

pub async fn get_historical_statistics_for_volume(
    id: String,
) -> Result<Vec<VolumeRow>, Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();
    println!("g off globule blan gro sayer la ");
    let mut cursor = client
    .query("SELECT timestamp AS ts, disk_usage AS dsk FROM volume_statistics WHERE id = ? ORDER BY timestamp ASC")
    .bind(id)
    .fetch::<VolumeRow>()?;

    let mut vector_response: Vec<VolumeRow> = vec![];

    while let Some(row) = cursor.next().await? {
        println!(
            "GET VOLUME ROOOOOOOOOOWWWWWW GAHHHHHHHRHHHHHHHHHHHHHHHHHHH TEST: {:?}",
            row
        );
        vector_response.push(row)
    }

    Ok(vector_response)
}
