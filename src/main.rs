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

async fn handle_hello() -> impl IntoResponse {
    println!("-> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>World!!!</strong>")
}
