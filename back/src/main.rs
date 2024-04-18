#![allow(non_snake_case)]

use std::sync::{atomic::AtomicUsize, Arc, Mutex};

use database::init_clickhouse_database;
use icons::spawn_info_service;
// use sniffer::sniff_packets;
use sqlitedb::init_sqlite_database;
use stats::{start_container_statistics_listeners, start_volume_statistics_listeners};
use tokio::time::{sleep, Duration};

use crate::volumes::common::get_all_volumes;

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
mod state;
mod stats;
mod topology;
mod utils;
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
        state::count_handler,
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

    let volume_state = Arc::new(Mutex::new(state::AllVolumesCacheState {
        last_update: 0,
        volumes: vec![],
    }));

    let external = volume_state.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            loop {
                let mut use_state = external.lock().unwrap();
                // let _ = update_volumes_state(&use_state);
                // println!("Use state: {:?}", use_state);

                let current_timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                use_state.last_update = current_timestamp.into();

                use_state.volumes = get_all_volumes().await.into();
                
                // unlock
                drop(use_state);


                sleep(Duration::from_secs(15)).await;
            }
        });
    });

    rocket::build()
        .manage(state::HitCount {
            count: AtomicUsize::new(0),
        })
        .manage(volume_state)
        .mount("/", all_handlers)
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
