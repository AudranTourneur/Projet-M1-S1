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
pub fn dns_list_handler() -> Json<DnsList> {
    let dns = DnsList {
        dns: vec![DnsData {
            domain_name: "example.com".to_string(),
            reverse_proxy_port: Some(8080),
        }],
    };

    Json(dns)
}

#[post("/dns/upsert", data = "<dns>")]
pub fn dns_upsert_handler(dns: Json<DnsData>) -> Json<bool> {
    Json(false)
}