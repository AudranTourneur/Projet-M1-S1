use bollard::{image::ListImagesOptions, Docker};
use regex::Regex;

use crate::{sqlitedb::get_sqlite_connection, web::get_url_response_cached};

use sqlx::Row;

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

    for image_name in image_names.split(", ") {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let mut conn = get_sqlite_connection().await.unwrap();

        let url = get_url_from_tag_name(&image_name);

        println!("Fetching image info for: {}", image_name);

        let query = sqlx::query("SELECT * FROM web_cache WHERE request_url = ?")
            .bind(&image_name)
            .fetch_all(&mut conn)
            .await
            .unwrap();

        if query.len() > 0 {
            println!("[SKIP] --- Image info already exists for {}", image_name);
            continue;
        }

        let response = get_url_response_cached(url).await;

        match response {
            Ok(_) => {
                println!("Successfully fetched image info for: {}", image_name);
            }
            Err(e) => {
                println!("Failed to fetch image info for: {}: {}", image_name, e);
                continue;
            }
        };
    }
}

fn get_url_from_tag_name(tag_name: &str) -> String {
    let before_colon = tag_name.split(":").collect::<Vec<&str>>()[0];
    let image_name = before_colon;

    if image_name.contains("/") {
        let before_slash = image_name.split("/").collect::<Vec<&str>>()[0];
        let after_slash = image_name.split("/").collect::<Vec<&str>>()[1];
        format!("{}/{}/{}", API_LINK, before_slash, after_slash)
    } else {
        format!("{}/{}/{}", API_LINK, "library", image_name)
    }
}

pub async fn resolve_icon_url_from_image_name(image_tag: &str) -> Option<String> {
    if image_tag == "<missing>" {
        return None;
    }

    let url = get_url_from_tag_name(&image_tag);
    println!(
        "Attempting to resolve icon for image {} | URL = {}",
        image_tag,
        url.clone()
    );

    let mut conn = get_sqlite_connection().await.unwrap();

    let db_res = sqlx::query("SELECT response_text FROM web_cache WHERE request_url = ?")
        .bind(url.clone())
        .fetch_all(&mut conn)
        .await;

    let db_res = match db_res {
        Ok(res) => res,
        Err(e) => {
            println!(
                "Failed to resolve icon for image {} (db error) | {}",
                image_tag, e
            );
            return None;
        }
    };

    let db_res = db_res.get(0);
    let db_res = match db_res {
        Some(res) => res,
        None => {
            println!(
                "Failed to resolve icon for image {} (no db response)",
                image_tag
            );
            return None;
        }
    };

    let text: String = db_res.get("response_text");

    let pattern = "!\\[logo\\]\\(([^\\n ]*)\\)";

    let re = Regex::new(pattern).unwrap();

    let caps = re.captures(&text);

    let caps = match caps {
        Some(res) => {
            println!("Resolved icon for image {}", image_tag);
            for i in 0..res.len() {
                println!("Match {}: {}", i, res.get(i).unwrap().as_str());
            }
            res
        }
        None => {
            println!(
                "Failed to resolve icon for image {} (no regex match) for URL {}",
                image_tag,
                url.clone()
            );
            return None;
        }
    };

    let icon_url = caps.get(1);

    let icon_url = match icon_url {
        Some(res) => res.as_str(),
        None => {
            println!(
                "Failed to resolve icon for image {} (no icon url)",
                image_tag
            );
            return None;
        }
    };

    println!("Resolved icon for image {}: {}", image_tag, icon_url);
    Some(icon_url.to_string())
}
