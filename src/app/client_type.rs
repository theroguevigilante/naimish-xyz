use axum::{http::HeaderMap, response::IntoResponse};

pub enum ClientType {
    Cli,
    Browser,
}

pub fn detect_client(headers: &HeaderMap) -> ClientType {
    let ua = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if ua.contains("curl") {
        ClientType::Cli
    } else {
        ClientType::Browser
    }
}

pub async fn handler(headers: HeaderMap) -> impl IntoResponse {
    match detect_client(&headers) {
        ClientType::Browser => "This is a browser",
        ClientType::Cli => "You are on a CLI",
    }
}
