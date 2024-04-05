use bollard::container::{Config, CreateContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures::StreamExt;
use rocket::serde::json::Json;
use crate::docker::get_docker_socket;
use crate::images::models::{HistoryResponse, ImageData, ImagePullRequest};
use super::common::get_all_images;
use super::models::{ImageCreateContainerRequest, ImageList};

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
        icon_url: Some("https://cdn.iconscout.com/icon/free/png-256/nginx-226046.png".into())
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

#[post("/images/create-container", data = "<input>")]
pub async fn create_container_from_image_handler(input: Json<ImageCreateContainerRequest>) -> Json<bool> {
    let docker: Docker = get_docker_socket();

    let user_image_name = input.image_name.clone();
    let user_container_name = input.container_name.clone();

    let create_options = match user_container_name {
        Some(container_name) => Some(
            CreateContainerOptions {
                name: container_name,
                ..Default::default()
            }
        ),
        None => None,
    };

    let res = docker.create_container(create_options,
        Config {
            image: Some(user_image_name),
            ..Default::default()
        }
    ).await;

    match res {
        Ok(_) => {
            println!("Container created successfully {:?}", res);
            Json(true)
        },
        Err(_) => Json(false),
    }
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