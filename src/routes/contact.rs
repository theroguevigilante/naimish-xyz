use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate;

pub async fn handler() -> Html<String> {
    Html(ContactTemplate.render().unwrap())
}
