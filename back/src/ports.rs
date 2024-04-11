use rocket::serde::json::Json;

#[derive(serde::Serialize)]
pub struct PortData {
    ip: String,
    port: u16,
}

#[derive(serde::Serialize)]
pub struct PortsResponse {
    ports: Vec<PortData>,
}

pub fn get_used_ports() -> Result<Vec<PortData>, std::io::Error> {
    let cmd = "netstat -tulpn | grep LISTEN";
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;
    
    let output = String::from_utf8(output.stdout).unwrap_or("".into());
    let lines: Vec<&str> = output.split("\n").collect();
    let mut ports: Vec<PortData> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let ip_port: Vec<&str> = parts[3].split(":").collect();
        let ip = ip_port[0];
        let port = ip_port[1].parse::<u16>().unwrap();
        ports.push(PortData {
            ip: ip.to_string(),
            port,
        });
    }

    Ok(ports)
}

#[get("/ports")]
pub fn ports_handler() -> Json<PortsResponse> {
    let ports = get_used_ports();
    let ports = match ports {
        Ok(ports) => ports,
        Err(e) => {
            println!("Error getting ports: {}", e);
            Vec::new()
        },
    };

    let res = PortsResponse { ports };

    Json(res)
}

// #[get("/is-port-open/<ip>/<port>")]
// pub async fn is_port_open(ip: String, port: u16) -> Json<bool> {
//     let res = tokio::task::spawn_blocking(move || {
//         let socket = std::net::SocketAddr::new(ip.parse().unwrap(), port);
//         let result = std::net::TcpStream::connect_timeout(&socket, std::time::Duration::from_secs(1));
//         result.is_ok()
//     }).await.unwrap();

//     Json(res)
// }
