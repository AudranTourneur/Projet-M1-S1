use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SimplePortData {
    ip: String,
    port: u16,
}

#[derive(Serialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PortsResponse {
    ports: Vec<SimplePortData>,
}

pub fn get_used_ports() -> Result<Vec<SimplePortData>, std::io::Error> {
    let cmd = "netstat -tulpn | grep LISTEN";
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;

    let output = String::from_utf8(output.stdout).unwrap_or("".into());
    let lines: Vec<&str> = output.split("\n").collect();
    let mut ports: Vec<SimplePortData> = Vec::new();

    for line in lines {
        println!("line: {}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let ip_port: Vec<&str> = parts[3].split(":").collect();
        let mut ip = ip_port[0];
        let port = ip_port[1].parse::<u16>();
        let port = match port {
            Ok(port) => port,
            Err(_) => {
                println!("Error parsing port: {}", parts[3]);

                let port = ip_port[3].parse::<u16>();
                match port {
                    Ok(port) => {
                        ip = "::";
                        port
                    }
                    Err(_) => {
                        println!("Error parsing port: {}", parts[3]);
                        continue;
                    }
                }
            }
        };
        ports.push(SimplePortData {
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
        }
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
