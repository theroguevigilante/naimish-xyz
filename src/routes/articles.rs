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

pub async fn list_handler() -> Html<String> {
    let page = ListTemplate{
        page_title: "articles",
        page_description: "Beep Boop",
        posts: recent_articles()
    };
    Html(page.render().unwrap())
}

pub async fn handler(Path(slug): Path<String>) -> impl IntoResponse {
    match get_post(&format!("articles/{}", slug)) {
        Some(post) => {
            let tmpl = PostTemplate {
                title: &post.summary.title,
                date: post.summary.date.as_deref().unwrap_or(""),
                content: &md_to_html(strip_front_matter(post.content.as_deref().unwrap_or(""))),
            };

            Html(tmpl.render().unwrap()).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
