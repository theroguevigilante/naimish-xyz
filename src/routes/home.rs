use crate::app::loader::md_to_html;
use crate::app::loader::{recent_posts, recent_ramblings};
use crate::app::types::Post;
use crate::app::client_type::{detect_client, ClientType, cli_card_handler};
use askama::Template;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    site_title: &'a str,
    page_name: &'a str,
    site_description: String,
    about_text: String,
    recent_posts: Vec<Post>,
    recent_ramblings: Vec<Post>,
}

pub async fn handler(headers: HeaderMap) -> impl IntoResponse {
    if let ClientType::Cli = detect_client(&headers) {
        return cli_card_handler().await.into_response();
    }

    let about_text = std::fs::read_to_string("content/about.md")
        .unwrap_or_default();
    let site_description = std::fs::read_to_string("content/description.txt")
        .unwrap_or_default();

    let mut ramblings = recent_ramblings();
    for post in &mut ramblings {
        if let Some(c) = &post.content {
            post.content = Some(md_to_html(c));
        }
    }

    let page = IndexTemplate {
        site_title: "Naimish",
        page_name: "naimish.xyz",
        site_description: site_description.trim().to_string(),
        about_text: md_to_html(&about_text),
        recent_posts: recent_posts(),
        recent_ramblings: ramblings,
    };
    Html(page.render().unwrap()).into_response()
}
