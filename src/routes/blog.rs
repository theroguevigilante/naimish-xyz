use crate::app::loader::recent_blogs;
use crate::app::types::Post;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate<'a> {
    pub page_title: &'a str,
    pub page_description: &'a str,
    pub posts: Vec<Post>,
}


pub async fn list_handler() -> Html<String> {
    let page = ListTemplate{
        page_title: "blog",
        page_description: "Beep Boop",
        posts: recent_blogs()
    };
    Html(page.render().unwrap())
}
pub async fn handler() {}
