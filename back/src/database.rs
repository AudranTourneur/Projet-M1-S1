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

#[derive(Row, Deserialize, Clone)]
struct VolumeLastAcquisitionResponseRow {
    max_timestamp: u64,
}

pub async fn get_volume_last_acquisition_timestamp(path: String) -> Result<u64, Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();

    let cursor = client
        .query("SELECT max(timestamp) AS max_timestamp FROM volume_statistics WHERE path = ?")
        .bind(&path)
        .fetch_optional::<VolumeLastAcquisitionResponseRow>()
        .await;

    let cursor = match cursor {
        Ok(cursor) => cursor,
        Err(e) => match e {
            clickhouse::error::Error::NotEnoughData => {
                println!("Not enough data for volume {}", &path);
                return Ok(0);
            }
            _ => {
                println!("Error getting volume acquisition timestamp: {}", e);
                return Err(Box::new(e));
            }
        },
    };

    let row = match cursor {
        Some(row) => row,
        None => {
            println!("No row found for volume {}", &path);
            return Ok(0);
        }
    };

    return Ok(row.max_timestamp);
}

pub async fn insert_volume_stats(volume_statistics: VolumeStats) -> Result<(), Box<dyn Error>> {
    let clickhouse_client = get_clickhouse_client();

    // let insert_query = format!(
    //     "INSERT INTO volume_statistics (path, timestamp, disk_usage)
    //     VALUES ('{}', '{}', '{}')",
    //     volume_statistics.path, volume_statistics.timestamp, volume_statistics.disk_usage,
    // );

    // SQL injection safe
    let insert_query =
        "INSERT INTO volume_statistics (path, volume_id, timestamp, disk_usage) VALUES (?, ?, ?, ?)";

    let vol_id = match volume_statistics.volume_id {
        Some(id) => id,
        None => String::from(""),
    };

    let insert = clickhouse_client
        .query(&insert_query)
        .bind(volume_statistics.path)
        .bind(vol_id)
        .bind(volume_statistics.timestamp)
        .bind(volume_statistics.disk_usage);

    let res = insert.execute().await;

    match res {
        Ok(_) => (),
        Err(e) => println!("Error inserting volume statistics: {}", e),
    };

    Ok(())
}

#[derive(Row, Deserialize, Serialize, Debug, TS)]
pub struct VolumeStatsRow {
    // pub path: String,
    // pub volume_id: String,
    // pub timestamp: u64,
    pub disk_usage: u64,
}

// pub async fn get_volume_latest_size(path: String) -> Result<u64, Box<dyn Error>> {
//     println!("Attempting to get volume size for {}", path.clone());
//     let client: clickhouse::Client = get_clickhouse_client();

//     let cursor = client
//         .query("SELECT disk_usage FROM volume_statistics WHERE path = ? ORDER BY timestamp DESC LIMIT 1")
//         .bind(path.clone())
//         .fetch_optional::<VolumeStatsRow>()
//         .await;

//     let cursor = match cursor {
//         Ok(cursor) => cursor,
//         Err(e) => {
//             match e {
//                 clickhouse::error::Error::NotEnoughData => {
//                     println!("Not enough data for volume {}", path);
//                     return Ok(0);
//                 },
//                 _ => {
//                     println!("Error getting volume acquisition timestamp: {}", e);
//                     return Err(Box::new(e));
//                 }
//             }
//         }
//     };

//     let row = match cursor {
//         Some(row) => row,
//         None => {
//             println!("No row found for volume {}", path);
//             return Ok(0);
//         }
//     };

//     return Ok(row.disk_usage as u64);
// }

#[derive(Row, Deserialize, Serialize, Debug, TS)]
pub struct DiskUsageRow {
    pub disk_usage: u64,
}

pub async fn get_volume_latest_size(path: String) -> u64 {
    let client: clickhouse::Client = get_clickhouse_client();

    let cursor = client
        .query(
            "SELECT disk_usage FROM volume_statistics WHERE path=? ORDER BY timestamp DESC LIMIT 1",
        )
        .bind(&path)
        .fetch::<DiskUsageRow>();

    let mut cursor = match cursor {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("Error getting volume acquisition disk_usage: {}", e);
            println!("get_volume_latest_size for path: {} --- 0 (error) {:?}", path.clone(), e);
            return 0;
        }
    };

    let mut vector_response: Vec<DiskUsageRow> = vec![];

    while let Ok(Some(row)) = cursor.next().await {
        vector_response.push(row)
    }

    if vector_response.len() == 0 {
        println!("No data found for volume {}", &path);
        return 0;
    }

    return vector_response[0].disk_usage;
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
