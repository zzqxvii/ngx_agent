mod api;
mod model;
mod service;
mod util;

use std::net::SocketAddr;
use std::process::Command;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = api::load_routers();

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
