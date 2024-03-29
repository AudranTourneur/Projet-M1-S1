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

#[get("/is-port-open/<ip>/<port>")]
pub async fn is_port_open(ip: String, port: u16) -> Json<bool> {
    let res = tokio::task::spawn_blocking(move || {
        let socket = std::net::SocketAddr::new(ip.parse().unwrap(), port);
        let result = std::net::TcpStream::connect_timeout(&socket, std::time::Duration::from_secs(1));
        result.is_ok()
    }).await.unwrap();

    Json(res)
}