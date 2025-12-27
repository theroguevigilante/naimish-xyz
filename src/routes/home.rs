use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    current_year: u16,
    recent_posts: Vec<Post>,
}

struct Post {
    slug: String,
    title: String,
    date: String,
}

pub async fn handler() -> Html<String> {
    let page = IndexTemplate {
        current_year: 2025,
        recent_posts: Vec::new(),
    };
    Html(page.render().unwrap())
}
