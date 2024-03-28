use bollard::image::CreateImageOptions;
use bollard::{image::ListImagesOptions, Docker};
use rocket::serde::{json::Json, Deserialize, Serialize};
use futures::prelude::stream::StreamExt;
use bollard::image::RemoveImageOptions;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct HistoryResponse {
    id: String,
    created: i64,
    created_by: String,
    tags: Vec<String>,
    size: i64,
    comment: String,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Image {
    id: String,
    tags: Vec<String>,
    size: i64,
    created: i64,
    history: Option<Vec<HistoryResponse>>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
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
                history: None,
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

#[get("/image/<id>")]
pub async fn image_handler(id: &str) -> Json<Image> {
    let all_images: Vec<Image> = get_all_images().await;
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

    let response = Image {
        id: image.id.clone(),
        tags: image.tags.clone(),
        size: image.size.clone(),
        created: image.created.clone(),
        history: Some(history),
    };

    Json(response)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImagePullRequest {
    pub id: String,
}

#[post("/images/pull", data = "<input>")]
pub async fn pull_image(input: Json<ImagePullRequest>) -> &'static str {

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


#[post("/images/<id>/remove")]
pub async fn delete_image(id :&str) -> &'static str{
    let docker : Docker = Docker::connect_with_local_defaults().unwrap();
    let options:Option<RemoveImageOptions> = Some(RemoveImageOptions {
        force: true,
        ..Default::default()
    });

    let _ = docker.remove_image(id, options, None).await;
    
    "Success."
}