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
        };
    } else {
        println!("Database already exists");
    }

    let query = include_str!("resources/init_db.sql");
    println!("Query: {}", query);
    let mut conn: SqliteConnection = SqliteConnection::connect(DB_URL).await.unwrap();

    let res = sqlx::query(query).execute(&mut conn).await;

    match res {
        Ok(_) => println!("Database initialized"),
        Err(e) => panic!("Error initializing database: {}", e),
    };

    Ok(())
}

pub async fn get_sqlite_connection() -> Result<SqliteConnection, Box<dyn Error>> {
    let conn = SqliteConnection::connect(DB_URL).await?;
    Ok(conn)
}
