use rocket::Route;

use crate::networks::handlers::{network_handler, networks_handler};

pub fn get_all_network_handlers() -> Vec<Route> {
    routes![networks_handler, network_handler,]
}
