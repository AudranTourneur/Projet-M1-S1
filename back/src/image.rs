use rocket::serde::{Serialize, Deserialize, json::Json}

#[derive(Serialize, Deserialize)]
pub struct ImageResponse {
    images: Vec<Image>
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    id: String,
    name: String,
    size: i32,
    created: i32,

    }

#[get("/images")]
pub async fn image_handler() -> Json<ImageResponse> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

    let images = &docker.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    let mut image : Vec<Image> = images.iter().map(|image| {
        let image_data = Image {
            id: image.id.clone(),
            name: image.name.clone(),
            size: image.size.clone(),
            created: image.created.clone(),
        };
        image_data
    }).collect();

    let response = ImageResponse {
        images: images,
    };

    Json(response)
}