mod app;
mod routes;

use axum::{Router, routing::get, routing::get_service};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

use app::client_type;
use routes::home;
use routes::articles;
use routes::blog;
use routes::contact;


#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "6432".into() );
    let addr = format!("127.0.0.1:{}", port);
    let router = Router::new()
        .route("/", get(home::handler))
        .nest(
            "/articles",
            Router::new()
                .route("/", get(articles::list_handler))
                .route("/{slug}", get(articles::handler))
        )
        .nest(
            "/blog",
            Router::new()
                .route("/", get(blog::list_handler))
                .route("/{slug}", get(blog::handler))
        )
        .route("/contact", get(contact::handler))
        .nest_service("/static", ServeDir::new("static"))
        .route_service("/naimish.asc", get_service(ServeFile::new("naimish.asc")))
        .route("/client_test", get(client_type::handler));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
