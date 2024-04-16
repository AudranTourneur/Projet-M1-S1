use crate::{docker::get_docker_socket, icons::resolve_icon_url_from_image_name};
use bollard::{image::ListImagesOptions, Docker};
use futures::future::join_all;

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

    let my_images: Vec<ImageData> = join_all(
        base_images
            .iter()
            .map(|image| async {
                let image = image.clone();

                let binding = "".to_string();
                let tag = image.repo_tags.get(0).unwrap_or(&binding);

                println!("Resolving img tag {}", tag);
                ImageData {
                    id: image.id.clone(),
                    tags: image.repo_tags.clone(),
                    size: image.size,
                    created: image.created,
                    history: None,
                    icon_url: resolve_icon_url_from_image_name(tag).await,
                }
            })
            .collect::<Vec<_>>(),
    )
    .await;

    my_images
}

// VERY SLOW
pub async fn get_image_by_id(id: &str, all_images: &Vec<ImageData>) -> Option<ImageData> {
    let img = all_images.iter().find(|image| image.id == id);
    
    img.map(|img| img.clone())
}
