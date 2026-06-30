use crate::app::loader::recent_projects;
use crate::app::types::Project;
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate<'a> {
    page_title: &'a str,
    projects: Vec<Project>,
}

pub async fn list_handler() -> Html<String> {
    let page = ProjectsTemplate {
        page_title: "projects",
        projects: recent_projects(),
    };
    Html(page.render().unwrap())
}

pub async fn handler(Path(_slug): Path<String>) -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}
