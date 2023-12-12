use std::{env, error::Error};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use dotenvy::dotenv;

#[derive(Debug, FromRow)]
struct Ticket {
	id: i64,
	name: String,
}

pub async fn create_pool() -> Result<PgPool, Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;

// 2) Create table if not exist yet
	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS ticket (
  id bigserial,
  name text
);"#,
	)
	.execute(&pool)
	.await?;

	// 3) Insert a new ticket
	let row: (i64,) = sqlx::query_as("insert into ticket (name) values ($1) returning id")
		.bind("a new ticket")
		.fetch_one(&pool)
		.await?;

	// 4) Select all tickets
	let rows = sqlx::query("SELECT * FROM ticket").fetch_all(&pool).await?;
	let str_result = rows
		.iter()
		.map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
		.collect::<Vec<String>>()
		.join(", ");
	println!("\n== select tickets with PgRows:\n{}", str_result);

	// 5) Select query with map() (build the Ticket manually)
	let select_query = sqlx::query("SELECT id, name FROM ticket");
	let tickets: Vec<Ticket> = select_query
		.map(|row: PgRow| Ticket {
			id: row.get("id"),
			name: row.get("name"),
		})
		.fetch_all(&pool)
		.await?;
	println!("\n=== select tickets with query.map...:\n{:?}", tickets);

	// 6) Select query_as (using derive FromRow)
	let select_query = sqlx::query_as::<_, Ticket>("SELECT id, name FROM ticket");
	let tickets: Vec<Ticket> = select_query.fetch_all(&pool).await?;
	println!("\n=== select tickets with query.map...: \n{:?}", tickets);

    Ok(pool)
}
