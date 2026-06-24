use axum::{
    Router,
};
use dotenv::dotenv;

use crate::router::create_router;

mod router;

async fn serve(app: Router, port: u16) {
    let addr = std::net::SocketAddr::from(([127,0,0,1],port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Axum server running on port {port}");
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = std::env::var("PORT").expect("PORT not defined").parse().unwrap();
    let app = create_router();
    serve(app, port).await;
}
