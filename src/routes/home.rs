use crate::app::loader::recent_posts;
use crate::app::types::Post;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    site_title: &'a str,
    page_name: &'a str,
    site_description: &'a str,
    recent_posts: Vec<Post>,
    num_of_posts: usize,
}

pub async fn handler() -> Html<String> {
    let page = IndexTemplate {
        site_title: "Naimish",
        page_name: "naimish.xyz",
        site_description: "This is my personal site. Rewritten in rust. As always I am going to be using this for documenting my projects. Have a good time :)",
        recent_posts: recent_posts(),
        num_of_posts: 5,
    };
    Html(page.render().unwrap())
}
