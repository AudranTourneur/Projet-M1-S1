use netstat::{AddressFamilyFlags, ProtocolFlags};
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

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let stats = netstat::get_sockets_info(af_flags, proto_flags);

    match stats {
        Ok(stats) => {
            println!("Found {} sockets.", stats.len());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let res = PortsResponse {
        ports: vec![]
    };

    Json(res)
}
