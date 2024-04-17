#![allow(non_snake_case)]

use database::init_clickhouse_database;
use icons::spawn_info_service;
// use sniffer::sniff_packets;
use sqlitedb::init_sqlite_database;
use stats::{start_container_statistics_listeners, start_volume_statistics_listeners};

mod auth;
mod containers;
mod docker;
mod images;
mod models;
mod networks;
mod overview;
//mod stats;
mod composes;
mod database;
mod dns;
mod icons;
mod ports;
// mod sniffer;
mod sqlitedb;
mod stats;
mod topology;
mod volumes;
mod web;



// #[macro_use]
// extern crate debug_stub_derive;

#[macro_use]
extern crate rocket;

fn create_rocket_app() -> rocket::Rocket<rocket::Build> {
    let base_routes = routes![
        auth::login_user_handler,
        auth::auth_me_handler,
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
        .cloned()
        .collect::<Vec<_>>();

    rocket::build().mount("/", all_handlers)
}

#[rocket::main]
async fn main() {
    let _ = init_clickhouse_database().await;

    let _ = init_sqlite_database().await;

    let app = create_rocket_app();

    rocket::tokio::spawn(start_container_statistics_listeners());
    rocket::tokio::spawn(start_volume_statistics_listeners());


    rocket::tokio::spawn(spawn_info_service());

    // rocket::tokio::spawn(sniff_packets());

    let _ = app.launch().await;
}
