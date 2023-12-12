mod auth;
mod containers;
mod images;
mod models;
mod networks;
mod overview;
mod schema;
//mod stats;
mod database;
mod topology;
mod volumes;
mod stats;

#[macro_use]
extern crate rocket;

fn create_rocket_app() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount(
        "/",
        routes![
            auth::auth_handler,
            overview::overview_handler,
            volumes::volumes_handler,
            images::images_handler,
            networks::networks_handler,
            containers::containers_handler,
            networks::network_handler,
            images::image_handler,
            containers::container_handler,
            volumes::volume_handler,
            containers::container_start,
            containers::container_stop,
        ],
    )
}

async fn spawn_statistics_subsystem() {
    stats::start_statistics_listeners().await;
}

#[rocket::main]
async fn main() {

    let app = create_rocket_app();

    rocket::tokio::spawn(spawn_statistics_subsystem());

    let _ = app.launch().await;
}
