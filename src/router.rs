use axum::{
    Router,
    routing::get
};

use crate::{handlers::users::get_user, middlewares::cors::cors_layer};

pub fn create_router() -> Router {
    Router::new()
    .route("/", get(|| async { "Hello World" }))
    .route("/users/{id}", get(get_user))
    .layer(cors_layer())
}