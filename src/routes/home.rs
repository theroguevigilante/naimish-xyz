use crate::app::loader::md_to_html;
use crate::app::loader::recent_posts;
use crate::app::types::Post;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    site_title: &'a str,
    page_name: &'a str,
    site_description: String,
    about_text: String,
    recent_posts: Vec<Post>,
    num_of_posts: usize,
}

pub async fn handler() -> Html<String> {
    let about_text = std::fs::read_to_string("content/about.md")
        .unwrap_or_default();
    let site_description = std::fs::read_to_string("content/description.txt")
        .unwrap_or_default();

    let page = IndexTemplate {
        site_title: "Naimish",
        page_name: "naimish.xyz",
        site_description: site_description.trim().to_string(),
        about_text: md_to_html(&about_text),
        recent_posts: recent_posts(),
        num_of_posts: 5,
    };
    Html(page.render().unwrap())
}
