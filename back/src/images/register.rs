use rocket::Route;
use crate::images::handlers::{create_container_from_image_handler, pull_image_handler};

use super::handlers::{image_handler, images_handler};

pub fn get_all_image_handlers() -> Vec<Route> {
    routes![
        image_handler,
        images_handler,
        pull_image_handler,
        create_container_from_image_handler
    ]
}