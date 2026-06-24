use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer}; 
use std::env;

pub fn cors_layer() -> CorsLayer {
    let origins = env::var("CORS_ORIGINS").expect("CORS_ORIGINS not defined");
    let parsed_origins: Vec<HeaderValue> = origins.split(';').map(|o| o.trim().parse().unwrap()).collect();

    CorsLayer::new()
    .allow_origin(AllowOrigin::list(parsed_origins))
    .allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE
    ])
}