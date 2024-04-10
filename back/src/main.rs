use database::init_clickhouse_database;
use sqlitedb::init_sqlite_database;
use icons::spawn_info_service;

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
mod sqlitedb;
mod icons;
mod composes;
mod dns;
mod web;

// #[macro_use]
// extern crate debug_stub_derive;

#[macro_use]
extern crate rocket;

fn create_rocket_app() -> rocket::Rocket<rocket::Build> {
    let base_routes =   routes![
            auth::auth_handler,
            overview::overview_handler,
            ports::ports_handler,
            topology::topology_handler,
            topology::topology_save_handler,
            composes::composes_handler,
            composes::compose_handler,
            composes::compose_start_handler,
            composes::compose_stop_handler,
            dns::dns_list_handler,
            // dns::dns_upsert_handler,
        ];

    let images_handlers = images::register::get_all_image_handlers();
    let containers_handlers = containers::register::get_all_container_handlers();
    let networks_handlers = networks::register::get_all_network_handlers();
    let volumes_handlers = volumes::register::get_all_volumes_handlers();

    let all_handlers = base_routes
        .iter()
        .chain(images_handlers.iter())
        .chain(containers_handlers.iter())
        .chain(networks_handlers.iter())
        .chain(volumes_handlers.iter())
        .map(|route| route.clone())
        .collect::<Vec<_>>();

    rocket::build().mount(
        "/",
        all_handlers
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

    rocket::tokio::spawn(spawn_info_service());

    let _ = app.launch().await;
}
