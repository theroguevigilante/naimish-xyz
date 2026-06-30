use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "donate.html")]
struct DonateTemplate;

pub async fn handler() -> Html<String> {
    let page = DonateTemplate;
    Html(page.render().unwrap())
}
