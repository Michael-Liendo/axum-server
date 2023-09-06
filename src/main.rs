use axum::extract::Query;
use serde::Deserialize;
use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handle_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on http://{addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}
