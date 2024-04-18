use rocket::Route;

use crate::volumes::handlers::{
    volume_explorer_handler, volume_handler, volumes_handler, volume_stats_handler,
};

pub fn get_all_volumes_handlers() -> Vec<Route> {
    routes![
        volumes_handler,
        volume_handler,
        volume_explorer_handler,
        volume_stats_handler,
    ]
}
