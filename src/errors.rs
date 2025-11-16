use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
struct ApiErrorResponse {
    message: String,
}

pub enum ApiError {
    BadRequest,
    DatabaseError(String),
    NotFound,
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.into()
    }
}

impl From<ApiError> for Response {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse {
                    message: "the request was malformed".to_string(),
                }),
            )
                .into_response(),
            ApiError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse { message: msg }),
            )
                .into_response(),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ApiErrorResponse {
                    message: "resource not found".to_string(),
                }),
            )
                .into_response(),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    message: "you are not authorized to access this content".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
