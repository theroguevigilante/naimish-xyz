use axum::{http::HeaderMap, response::IntoResponse};
use crate::app::loader::recent_ramblings;

pub enum ClientType {
    Cli,
    Browser,
}

pub fn detect_client(headers: &HeaderMap) -> ClientType {
    let ua = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if ua.contains("curl") || ua.contains("wget") {
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

pub async fn cli_card_handler() -> impl IntoResponse {
    let posts = recent_ramblings();
    let mut recent_text = String::new();
    for post in posts.iter().take(3) {
        recent_text.push_str(&format!("  * {} ({})\n", post.summary.title, post.summary.date_str()));
    }
    if recent_text.is_empty() {
        recent_text.push_str("  (No recent updates)\n");
    }

    let ascii = format!(
        "\n\
         \x1b[1;32m              _           _     _     \x1b[0m\n\
        \x1b[1;32m  _ __   __ _(_)_ __ ___ (_)___| |__  \x1b[0m\n\
        \x1b[1;32m | '_ \\ / _` | | '_ ` _ \\| / __| '_ \\ \x1b[0m\n\
        \x1b[1;32m | | | | (_| | | | | | | | \\__ \\ | | |\x1b[0m\n\
        \x1b[1;32m |_| |_|\\__,_|_|_| |_| |_|_|___/_| |_|\x1b[0m\n\
        \n\
        \x1b[1;36mAbout Me\x1b[0m\n\
        Recreational Programming. (loosely speaking I break things for fun)\n\
        \n\
        \x1b[1;36mSocials & Links\x1b[0m\n\
        GitHub:   \x1b[94mhttps://github.com/naimish\x1b[0m\n\
        Website:  \x1b[94mhttps://naimish.xyz\x1b[0m\n\
        PGP Key:  \x1b[94mnaimish.xyz/naimish.asc\x1b[0m\n\
        \n\
        \x1b[1;36mBrowse the Terminal Site\x1b[0m\n\
        Articles:  \x1b[33mnaimish.xyz/articles\x1b[0m\n\
        Blog:      \x1b[33mnaimish.xyz/blog\x1b[0m\n\
        Ramblings: \x1b[33mnaimish.xyz/ramblings\x1b[0m\n\
        \n\
        \x1b[1;36mRecent Ramblings\x1b[0m\n\
        {}\n",
        recent_text
    );

    ascii
}
