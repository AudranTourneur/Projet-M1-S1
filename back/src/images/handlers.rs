use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures::StreamExt;
use rocket::serde::json::Json;
use crate::images::models::{HistoryResponse, ImageData, ImagePullRequest};
use super::common::get_all_images;
use super::models::ImageList;

#[get("/images")]
pub async fn images_handler() -> Json<ImageList> {
    let my_images = get_all_images().await;
    let response = ImageList { images: my_images };

    Json(response)
}

#[get("/image/<id>")]
pub async fn image_handler(id: &str) -> Json<ImageData> {
    let all_images: Vec<ImageData> = get_all_images().await;
    let image = all_images.iter().find(|image| image.id == id).unwrap();
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    println!("history: {:?}", &docker.image_history(&image.id).await);

    //Collecting history
    let whole_history = &docker.image_history(&image.id).await.unwrap();
    let history = whole_history
        .iter()
        .map(|history| {
            let history_data = HistoryResponse {
                id: history.id.clone(),
                created: history.created.clone(),
                created_by: history.created_by.clone(),
                tags: history.tags.clone(),
                size: history.size,
                comment: history.comment.clone(),
            };
            history_data
        })
        .collect();

    let response = ImageData {
        id: image.id.clone(),
        tags: image.tags.clone(),
        size: image.size.clone(),
        created: image.created.clone(),
        history: Some(history),
    };

    Json(response)
}

#[post("/images/pull", data = "<input>")]
pub async fn pull_image_handler(input: Json<ImagePullRequest>) -> &'static str {

    println!("pull_image input: {:?}", input);

    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    let options = Some(CreateImageOptions {
        from_image: input.id.clone(),
        tag: "latest".to_string(),
        ..Default::default()
    });

    let mut stream = docker.create_image(options, None, None);

    while let Some(item) = stream.next().await {
        println!("stream iter {:?}", item);
    }

    "Success."
}


// #[post("/images/<id>/remove")]
// pub async fn delete_image(id :&str) -> &'static str{
//     let docker : Docker = Docker::connect_with_local_defaults().unwrap();
//     let options:Option<RemoveImageOptions> = Some(RemoveImageOptions {
//         force: true,
//         ..Default::default()
//     });

//     let _ = docker.remove_image(id, options, None).await;
    
//     "Success."
// }