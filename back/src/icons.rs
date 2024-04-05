use bollard::{image::ListImagesOptions, Docker};
use regex::Regex;

use crate::sqlitedb::get_sqlite_connection;

static API_LINK: &str = "https://hub.docker.com/v2/repositories";

pub async fn get_all_image_names() -> String {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let base_images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let image_names: Vec<String> = base_images
        .iter()
        .flat_map(|image| image.repo_tags.clone())
        .collect();

    image_names.join(", ")
}

pub async fn spawn_info_service() {
    let image_names = get_all_image_names().await;

    let client = reqwest::Client::new();

    //let conn = Connection::open(DATABASE_PATH).unwrap();

    for image_name in image_names.split(", ") {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let before_colon = image_name.split(":").collect::<Vec<&str>>()[0];

        let image_name = before_colon;

        let url = if image_name.contains("/") {
            let before_slash = image_name.split("/").collect::<Vec<&str>>()[0];
            let after_slash = image_name.split("/").collect::<Vec<&str>>()[1];
            format!("{}/{}/{}", API_LINK, before_slash, after_slash)
        } else {
            format!("{}/{}/{}", API_LINK, "library", image_name)
        };

        println!("URL {}", url);

        let mut conn = get_sqlite_connection().await.unwrap();

        let query = sqlx::query("SELECT * FROM images WHERE image_name = ?")
            .bind(&image_name)
            .fetch_all(&mut conn)
            .await
            .unwrap();

        if query.len() > 0 {
            println!("[SKIP] --- Image info already exists for {}", image_name);
            continue;
        }

        let response = client.get(&url).send().await;

        if response.is_err() {
            println!("Failed to fetch image info for: {}", image_name);
            continue;
        }

        let response = response.unwrap();

        let status = response.status();
        let response_text = response.text().await.unwrap();

        let res = if !status.is_success() {
            println!("Failed to fetch image info for: {}", image_name);
            sqlx::query("INSERT INTO images (image_name, docker_hub_response, docker_hub_status) VALUES (?, ?, ?)")
                .bind(image_name)
                .bind("")
                .bind(status.as_u16())
            .execute(&mut conn).await
        } else {
            println!("Fetched image info for: {}", image_name);
            sqlx::query("INSERT INTO images (image_name, docker_hub_response, docker_hub_status) VALUES (?, ?, ?)")
                .bind(image_name)
                .bind(response_text)
                .bind(status.as_u16())
            .execute(&mut conn).await
        };

        match res {
            Ok(_) => println!("Inserted image info for: {}", image_name),
            Err(e) => println!("Failed to insert image info for: {}: {}", image_name, e),
        }


        // if response.status().is_success() {
        //     let json_response: serde_json::Value = json!(response.text().await.unwrap());
        //     println!("{:?}", json_response);

        //let full_description = json_response.to_string();

        //conn.execute(
        //    "INSERT INTO images (image_name, docker_hub_response) VALUE (?1, ?2)",
        //    &[&image_name, &full_description],
        //)
        //.bind(&image_name)
        //.bind(&full_description)
        //.execute(&conn)
        //.await
        //.unwrap();
        // };

        // break;
    }
}


pub async fn resolve_icon_url_from_image_name(image: String) -> Option<String> {

    // let mut conn = get_sqlite_connection().await.unwrap();

    // let db_res = sqlx::query("SELECT docker_hub_response FROM images WHERE image_name = ?")
    //     .bind(image)
    //     .fetch_all(&mut conn)
    //     .await
    //     .unwrap();

    // if db_res.is_empty() {
    //     return None;
    // }


    // // let text: String = db_res[0].try_get("docker_hub_response").ok()?;

    // let pattern = "\\[!logo\\]\\((.*)\\)";

    // let re = Regex::new(pattern).unwrap();

    // if let Some(caps) = re.captures(&text) {
    //     if let Some(url) = caps.get(1) {
    //         return Some(url.as_str().to_string());
    //     }
    // }

    None

}