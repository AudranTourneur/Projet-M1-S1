use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub fn to_base64_url(data: &str) -> String {
    URL_SAFE.encode(data.as_bytes())
}

pub fn from_base64_url(data: &str) -> String {
    let vec: Vec<u8> = URL_SAFE.decode(data).unwrap();
    String::from_utf8(vec).unwrap()
}