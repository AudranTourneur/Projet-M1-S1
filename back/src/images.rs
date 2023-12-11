use bollard::{image::ListImagesOptions, Docker};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    id: String,
    tags: Vec<String>,
    size: i64,
    created: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageResponse {
    images: Vec<Image>,
}

async fn get_all_images() -> Vec<Image> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let base_images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let my_images: Vec<Image> = base_images
        .iter()
        .map(|image| {
            let image_data = Image {
                id: image.id.clone(),
                tags: image.repo_tags.clone(),
                size: image.size.clone(),
                created: image.created.clone(),
            };
            image_data
        })
        .collect();

    my_images
}

#[get("/images")]
pub async fn images_handler() -> Json<ImageResponse> {
    let my_images = get_all_images().await;
    let response = ImageResponse { images: my_images };

    Json(response)
}

#[get("/images/<id>")]
pub async fn image_handler(id: &str) -> Json<Image> {
    let all_images : Vec<Image> = get_all_images().await;
    let image = all_images.iter().find(|image| image.id == id).unwrap();

    let response = Image {
        id: image.id.clone(),
        tags: image.tags.clone(),
        size: image.size.clone(),
        created: image.created.clone(),
    };

    Json(response)

}
