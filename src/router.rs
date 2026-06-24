use axum::{
    Router,
    routing::get
};

use crate::middlewares::cors::cors_layer;

pub fn create_router() -> Router {
    Router::new()
    .route("/", get(|| async { "Hello World" }))
    .layer(cors_layer())
}