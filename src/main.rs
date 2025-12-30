mod app;
mod routes;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use app::client_type;
use routes::home;
use routes::articles;
use routes::blog;
use routes::contact;


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6432";
    let router = Router::new()
        .route("/", get(home::handler))
        .nest(
            "/articles",
            Router::new()
                .route("/", get(articles::list_handler))
//                .route("/:slug", get(articles::handler))
        )
        .nest(
            "/blog",
            Router::new()
                .route("/", get(blog::list_handler))
                .route("/{slug}", get(blog::handler))
        )
        .route("/contact", get(contact::handler))
        .nest_service("/static", ServeDir::new("static"))
        .route("/client_test", get(client_type::handler));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
