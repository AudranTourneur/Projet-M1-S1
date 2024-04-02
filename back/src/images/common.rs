use bollard::{image::ListImagesOptions, Docker};
use super::models::ImageData;

pub async fn get_all_images() -> Vec<ImageData> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let base_images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let my_images: Vec<ImageData> = base_images
        .iter()
        .map(|image| {
            let image_data = ImageData {
                id: image.id.clone(),
                tags: image.repo_tags.clone(),
                size: image.size.clone(),
                created: image.created.clone(),
                history: None,
            };
            image_data
        })
        .collect();

    my_images
}

