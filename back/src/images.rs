use bollard::{Docker, image::ListImagesOptions};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct Image {
    id: String,
    size: i64,
    created: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ImageResponse {
    images: Vec<Image>
}

#[get("/images")]
pub async fn images_handler() -> Json<ImageResponse> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

    let base_images = &docker.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    let my_images : Vec<Image> = base_images.iter().map(|image| {
        let image_data = Image {
            id: image.id.clone(),
            size: image.size.clone(),
            created: image.created.clone(),
        };
        image_data
    }).collect();

    let response = ImageResponse {
        images: my_images,
    };

    Json(response)
}