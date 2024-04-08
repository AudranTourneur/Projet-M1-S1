use crate::images::get_all_image_names;
//use sqlx::sqlite::SqliteConnection;
//use clickhouse::Client as ClickhouseClient;
//use sqlx::Connection;

static API_LINK: &str = "https://hub.docker.com/v2/repositories/library/";  
static NAMES: &str = "";

static DATABASE_PATH: &str = "/init_db.sql";

pub async fn spawn_infos_service() {
    let image_names = get_all_image_names().await;

    let client = reqwest::Client::new();

    //let conn = Connection::open(DATABASE_PATH).unwrap();

    for image_name in image_names.split(", "){
        let url = format!("{}{}", API_LINK, image_name);

        let response = client.get(&url).send().await.unwrap();

        if response.status().is_success() {
            //let json_response: serde_json::Value = response.json().await.unwrap();
            
            //let full_description = json_response.to_string();
            
            //conn.execute(
            //    "INSERT INTO images (image_name, docker_hub_response) VALUE (?1, ?2)",
            //    &[&image_name, &full_description],
            //)
            //.bind(&image_name)
            //.bind(&full_description)
            //.execute(&conn)
            //.await
            //.unwrap();
        }
    }

    


}