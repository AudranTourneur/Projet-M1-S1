use crate::sqlitedb::get_sqlite_connection;
use sqlx::Row;

pub async fn get_url_response_cached(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut conn = get_sqlite_connection().await?;

    let res = sqlx::query("SELECT * FROM web_cache WHERE request_url = ?")
        .bind(url.clone())
        .fetch_all(&mut conn)
        .await?;

    let row = res.first();
    match row {
        Some(res) => {
            let response_text: String = res.get("response_text");
            Ok(response_text)
        }
        None => {
            println!("No such cached response for URL: {}", url.clone());

            let web_client = reqwest::Client::new();
            let response = web_client.get(url.clone()).send().await?;
            let status = &response.status().as_u16();
            let body = response.text().await?;

            let current_timestamp_ms = chrono::Utc::now().timestamp_millis();

            sqlx::query("INSERT INTO web_cache (request_url, last_updated_at, response_status, response_text) VALUES (?, ?, ?, ?)")
                    .bind(url.clone())
                    .bind(current_timestamp_ms)
                    .bind(status)
                    .bind(&body)
                    .execute(&mut conn)
                    .await?;

            Ok(body)
        }
    }
}
