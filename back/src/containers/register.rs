use rocket::Route;

pub fn get_all_container_handlers() -> Vec<Route> {
    routes![
        crate::containers::handlers::containers_handler,
        crate::containers::handlers::container_handler,
        crate::containers::handlers::rebind_ports_handler,
        crate::containers::handlers::container_start,
        crate::containers::handlers::container_stop,
        crate::containers::handlers::container_stats_handler,
        crate::containers::handlers::delete_container,
        crate::containers::handlers::container_stats_stream_hander,
    ]
}
