use sqlx::sqlite::SqliteConnection;
use std::error::Error;

use sqlx::Connection;
use sqlx::{migrate::MigrateDatabase, Sqlite};


const DB_URL: &str = "sqlite://sqlite/db";

pub async fn init_sqlite_database() -> Result<(), Box<dyn Error>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let query = include_str!("sqlite/init_db.sql");
    println!("Query: {}", query);
    // let mut conn: SqliteConnection = SqliteConnection::connect(DB_URL).await?;
    // sqlx::query(query).execute(&mut conn).await?;

    println!("SQLite database initialized!");

    Ok(())
}
