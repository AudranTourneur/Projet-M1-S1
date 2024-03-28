use database::init_clickhouse_database;
use db::init_sqlite_database;

mod docker;
mod auth;
mod containers;
mod images;
mod models;
mod networks;
mod overview;
mod schema;
//mod stats;
mod database;
mod stats;
mod topology;
mod volumes;
mod ports;
mod db;

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
            containers::rebind_ports_handler,
            containers::container_filesystem_handler,
            volumes::volume_handler,
            containers::container_start,
            containers::container_stop,
            images::pull_image,
            containers::container_stats_handler,
            images::delete_image,
            containers::delete_container,
            volumes::delete_volume,
            containers::container_stats_stream_hander,
            ports::ports_handler,
            topology::topology_handler,
            topology::topology_save_handler,
        ],
    )
}

async fn spawn_statistics_subsystem() {
    stats::start_statistics_listeners().await;
}

#[rocket::main]
async fn main() {
    let _ = init_clickhouse_database().await;

    let _ = init_sqlite_database().await;

    let app = create_rocket_app();

    rocket::tokio::spawn(spawn_statistics_subsystem());

    let _ = app.launch().await;
}
