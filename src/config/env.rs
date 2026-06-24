use axum::http::HeaderValue;
use dotenv::dotenv;
use std::{env, sync::LazyLock};

pub struct Env {
    pub port: u16,
    pub cors_origins: Vec<HeaderValue>,
}

impl Env {
    pub fn load() -> Self {
        dotenv().ok();
        Self {
            port: env::var("PORT").unwrap_or_else(|_| "3001".into()).parse().unwrap(),
            cors_origins: required_env("CORS_ORIGINS").split(',').map(|o| o.trim().parse().unwrap()).collect(),
        }
    }
}

fn required_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} not defined"))
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| Env::load());