use crate::app::loader::get_post;
use crate::app::loader::recent_articles;
use crate::app::loader::strip_front_matter;
use crate::app::loader::md_to_html;
use crate::app::types::Post;
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate<'a> {
    pub page_title: &'a str,
    pub page_description: &'a str,
    pub posts: Vec<Post>,
}

#[derive(askama::Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub content: &'a str,
}

use crate::app::client_type::{detect_client, ClientType};
use axum::http::HeaderMap;

pub async fn list_handler(headers: HeaderMap) -> impl IntoResponse {
    let posts = recent_articles();
    
    if let ClientType::Cli = detect_client(&headers) {
        let mut output = format!("\x1b[1;32m=== ARTICLES ===\x1b[0m\n\n");
        for post in &posts {
            output.push_str(&format!("* {} ({})\n", post.summary.title, post.summary.date_str()));
            output.push_str(&format!("  naimish.xyz/{}{}\n\n", post.kind_route(), post.summary.slug));
        }
        return output.into_response();
    }

    let page = ListTemplate{
        page_title: "articles",
        page_description: "Beep Boop",
        posts
    };
    Html(page.render().unwrap()).into_response()
}

pub async fn handler(headers: HeaderMap, Path(slug): Path<String>) -> impl IntoResponse {
    match get_post(&format!("articles/{}", slug)) {
        Some(post) => {
            if let ClientType::Cli = detect_client(&headers) {
                let markdown_content = strip_front_matter(post.content.as_deref().unwrap_or(""));
                let output = format!(
                    "\x1b[1;32m# {}\x1b[0m\n\x1b[2m{}\x1b[0m\n\n{}", 
                    post.summary.title, 
                    post.summary.date_str(),
                    markdown_content
                );
                return output.into_response();
            }

            let tmpl = PostTemplate {
                title: &post.summary.title,
                date: post.summary.date.as_deref().unwrap_or(""),
                content: &md_to_html(strip_front_matter(post.content.as_deref().unwrap_or(""))),
            };

            Html(tmpl.render().unwrap()).into_response()
        }
        None => {
            if let ClientType::Cli = detect_client(&headers) {
                return (StatusCode::NOT_FOUND, "404 Not Found\n").into_response();
            }
            StatusCode::NOT_FOUND.into_response()
        },
    }
}
