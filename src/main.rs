use axum::extract::{Path, Query};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on http://{addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /hello?name=Michael
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}

// /hello2/michael
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-> {:<12} - handler_hello", "HANDLER");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}
