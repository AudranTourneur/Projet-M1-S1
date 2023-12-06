mod auth;
mod overview;
mod volumes;
mod images;
mod networks;

#[macro_use] extern crate rocket;
use bollard::Docker;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct MyVersion {
    version: String,
    api_version: String,
}

#[get("/version")]
async fn get_version() -> Json<MyVersion> {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();

    let version_object = docker.version().await.unwrap();

    let version_str = version_object.version.unwrap_or("UNDEFINED".to_string());
    let api_version_str = version_object.api_version.unwrap_or("UNDEFINED".to_string());

    let my_version = MyVersion {
        version: version_str,
        api_version: api_version_str,
    };

    return Json(my_version);
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_version, hello, auth::auth_handler, overview::overview_handler, volumes::volumes_handler, images::images_handler, networks::networks_handler])
}
