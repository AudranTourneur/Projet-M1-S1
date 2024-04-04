use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DnsData {
    pub domain_name: String,
    pub reverse_proxy_port: Option<u16>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DnsList {
    pub dns: Vec<DnsData>,
}

#[get("/dns")]
pub async fn dns_list_handler() -> Json<DnsList> {
    //let caddyfile_content = std::fs::read_to_string("/etc/caddy/Caddyfile").unwrap();    
    let client = reqwest::Client::new();

    // let url = "http://localhost:2019/config/apps/http/servers/srv0/routes";
    let url = "http://localhost:2019/config";

    let response = client.get(url).send().await.unwrap().text().await.unwrap();

    println!("{:?}", response);

    let dns = DnsList {
        dns: vec![]
    };

    Json(dns)
}

#[post("/dns/upsert", data = "<dns>")]
pub fn dns_upsert_handler(dns: Json<DnsData>) -> Json<bool> {
    Json(false)
}