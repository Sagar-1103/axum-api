use axum::{
    Router,
};

use crate::{config::env::ENV, router::create_router};

mod router;
mod middlewares;
mod config;
mod utils;
mod handlers;

async fn serve(app: Router, port: u16) {
    let addr = std::net::SocketAddr::from(([127,0,0,1],port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Axum server running on port {port}");
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    let app = create_router();
    serve(app, ENV.port).await;
}
