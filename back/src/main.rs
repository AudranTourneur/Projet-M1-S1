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

/*
async fn spawn_statistics_subsystem() {
    stats::start_statistics_listeners().await;
}
*/

#[rocket::main]
async fn main() {
    /*
    let mut conn = database::establish_connection();

    let user_form: UserForm = UserForm {
        username: "Bob".into(),
        password: "123".into(),
        salt: "123".into(),
        topology: "aaa".into(),
        updated_at: Default::default(),
    };

    let _ = diesel::insert_into(users)
        .values(&user_form)
        .execute(&mut conn);
    */

    let _ = database::create_pool().await;

    let app = create_rocket_app();
    let _ = app.launch().await;
    //rocket::tokio::spawn(app.launch());
    //spawn_statistics_subsystem().await;
}
