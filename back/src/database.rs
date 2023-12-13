use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{env, error::Error};

use crate::models::ContainerStats;

pub async fn create_pool() -> Result<PgPool, Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn insert_container_stats(
    container_statistics: ContainerStats,
) -> Result<(), Box<dyn Error>> {
    // let pool = create_pool().await.unwrap();

    // let _ = sqlx::query!(
    //     r#"
    //     INSERT INTO container_statistics (id, timestamp, cpu_usage, memory_usage, io_usage_read, io_usage_write, network_usage_up, network_usage_down)
    //     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    //     "#,
    //     container_statistics.container_id,
    //     container_statistics.timestamp,
    //     container_statistics.cpu_usage,
    //     container_statistics.memory_usage,
    //     container_statistics.io_usage_read,
    //     container_statistics.io_usage_write,
    //     container_statistics.network_usage_up,
    //     container_statistics.network_usage_down,
    // )
    // .execute(&pool)
    // .await;

    let clickhouse_client = get_clickhouse_client();

    let insert_query = format!(
        "INSERT INTO container_statistics (id, timestamp, cpu_usage, memory_usage, io_usage_read, io_usage_write, network_usage_up, network_usage_down)
        VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
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
        Ok(_) => println!("Container statistics inserted"),
        Err(e) => println!("Error inserting container statistics: {}", e),
    };

    Ok(())
}

use clickhouse::Client;

pub fn get_clickhouse_client() -> Client {
    Client::default()
        .with_url("http://localhost:8123")
        .with_user("username")
        .with_password("password")
        .with_database("my_database")
}

pub async fn init_clickhouse_database() -> Result<(), Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();

    let init_query = include_str!("../resources/clickhouse_init.sql");

    let create_table = client.query(init_query);

    let res = create_table.execute().await;

    println!("Table created");

    match res {
        Ok(_) => println!("Clickhouse database initialized"),
        Err(e) => panic!("Error initializing clickhouse database: {}", e),
    };

    Ok(())
}

use serde::{Deserialize, Serialize};
use clickhouse::Row;

#[derive(Row, Deserialize, Serialize, Debug)]
pub struct MyRow {
    ts: u32,
    mem: u64,
    cpu: f32,
}

pub async fn get_historical_statistics_for_container(
    id: String,
) -> Result<Vec<MyRow>, Box<dyn Error>> {
    let client: clickhouse::Client = get_clickhouse_client();

    let mut cursor = client
    .query("SELECT timestamp AS ts, memory_usage AS mem, cpu_usage AS cpu FROM container_statistics WHERE id = ?")
    .bind(id)
    .fetch::<MyRow>()?;

    let mut vector_response: Vec<MyRow> = vec![];

    while let Some(row) = cursor.next().await? {
        vector_response.push(row)
    }

    Ok(vector_response)
}
