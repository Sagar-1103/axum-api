use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::utils::response::ApiResponse;

#[derive(Debug,Error)]
pub enum ApiError {
    #[error("Internal server error")]
    Internal
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body:ApiResponse<()> = ApiResponse {
            success: false,
            message: self.to_string(),
            data: None
        };
        (self.status(),Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<(StatusCode,Json<ApiResponse<T>>),ApiError>;