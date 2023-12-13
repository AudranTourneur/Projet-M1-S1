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
    let pool = create_pool().await.unwrap();

    let _ = sqlx::query!(
        r#"
        INSERT INTO container_statistics (id, timestamp, cpu_usage, memory_usage, io_usage_read, io_usage_write, network_usage_up, network_usage_down)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        container_statistics.container_id,
        container_statistics.timestamp,
        container_statistics.cpu_usage,
        container_statistics.memory_usage,
        container_statistics.io_usage_read,
        container_statistics.io_usage_write,
        container_statistics.network_usage_up,
        container_statistics.network_usage_down,
    )
    .execute(&pool)
    .await;

    Ok(())
}
