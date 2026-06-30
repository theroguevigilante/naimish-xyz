use crate::app::loader::{get_project, md_to_html, recent_projects, strip_front_matter};
use crate::app::types::Project;
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate<'a> {
    page_title: &'a str,
    projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectTemplate<'a> {
    title: &'a str,
    content: &'a str,
    date: &'a str,
    tech: &'a str,
    repo: &'a str,
    docs: &'a str,
    description: &'a str,
}

pub async fn list_handler() -> Html<String> {
    let page = ProjectsTemplate {
        page_title: "projects",
        projects: recent_projects(),
    };
    Html(page.render().unwrap())
}

pub async fn handler(Path(slug): Path<String>) -> impl IntoResponse {
    if slug == "naimish-xyz" {
        return Redirect::to("/").into_response();
    }

    match get_project(&slug) {
        Some(project) => {
            let tmpl = ProjectTemplate {
                title: &project.title,
                content: &md_to_html(strip_front_matter(project.content.as_deref().unwrap_or(""))),
                date: project.date.as_deref().unwrap_or(""),
                tech: project.tech.as_deref().unwrap_or(""),
                repo: project.repo.as_deref().unwrap_or(""),
                docs: project.docs.as_deref().unwrap_or(""),
                description: project.description.as_deref().unwrap_or(""),
            };
            Html(tmpl.render().unwrap()).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
