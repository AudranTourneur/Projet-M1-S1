use rocket::serde::json::Json;

#[derive(serde::Serialize)]
pub struct PortData {
    ip: String,
    port: u16
}

#[derive(serde::Serialize)]
pub struct PortsResponse {
    ports: Vec<PortData>,
}

#[get("/ports")]
pub fn ports_handler() -> Json<PortsResponse> {
    let ports = vec![
        PortData {
            ip: "0.0.0.0".to_string(),
            port: 8080
        },
        PortData {
            ip: "0.0.0.0".to_string(),
            port: 3306
        },
    ];
  
    let res = PortsResponse {
        ports
    };

    Json(res)
}
