use rocket::Route;

use crate::volumes::handlers::{volume_explorer_handler, volume_handler, volumes_handler, delete_volume};

pub fn get_all_volumes_handlers() -> Vec<Route> {
    routes![
        volumes_handler,
        volume_handler,
        delete_volume,
        volume_explorer_handler,
    ]
}