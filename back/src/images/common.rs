use bollard::{image::ListImagesOptions, Docker};
use crate::docker::get_docker_socket;

use super::models::ImageData;

pub async fn get_all_images() -> Vec<ImageData> {
    let docker: Docker = get_docker_socket();
    let base_images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap_or(Vec::new());

    let my_images: Vec<ImageData> = base_images
        .iter()
        .map(|image| {
            let image_data = ImageData {
                id: image.id.clone(),
                tags: image.repo_tags.clone(),
                size: image.size.clone(),
                created: image.created.clone(),
                history: None,
                icon_url: Some("https://cdn.iconscout.com/icon/free/png-256/nginx-226046.png".into()),
            };
            image_data
        })
        .collect();

    my_images
}

