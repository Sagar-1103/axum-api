use axum::{Json, extract::Path, http::StatusCode};
use serde::Serialize;
use crate::utils::{ApiResult, error::ApiError, response::ApiResponse};

#[derive(Serialize)]
pub struct User {
    id: u32,
    name:String,
}

pub async fn get_user(Path(id): Path<u32>) -> ApiResult<User> {
    let succeed = id > 11;

    if succeed {
        let user = User {
            id,
            name:String::from("Sagar")
        };
    
        let response = ApiResponse::success("Fetched user successfully", user);
        Ok((StatusCode::OK,Json(response)))
    } else {
        Err(ApiError::Internal)
    }
}