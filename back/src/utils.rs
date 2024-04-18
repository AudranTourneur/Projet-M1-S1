use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub fn to_base64_url(data: &str) -> String {
    URL_SAFE.encode(data.as_bytes())
}

pub fn from_base64_url(data: &str) -> Option<String> {

    let copy = data.to_string();
    // add padding if needed
    let padding = match copy.len() % 4 {
        0 => 0,
        2 => 2,
        3 => 1,
        _ => return None,
    };

    let padding = "=".repeat(padding);
    let data = format!("{}{}", data, padding);

    let maybe_vec = URL_SAFE.decode(data);
    let vec = match maybe_vec {
        Ok(vec) => vec,
        Err(_) => return None,
    };
    let str = String::from_utf8(vec);

    match str {
        Ok(str) => Some(str),
        Err(_) => None,
    }
}