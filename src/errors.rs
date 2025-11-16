use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
struct ApiErrorResponse<'a> {
    message: &'a str,
    detail: Option<String>,
}

pub enum ApiError {
    BadRequest,
    DatabaseError(String),
    NotFound,
    Unauthorized(Option<String>),
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
                    message: "the request was malformed",
                    detail: None,
                }),
            )
                .into_response(),
            ApiError::DatabaseError(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    message: "database error",
                    detail: Some(detail),
                }),
            )
                .into_response(),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ApiErrorResponse {
                    message: "resource not found",
                    detail: None,
                }),
            )
                .into_response(),
            ApiError::Unauthorized(detail) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    message: "you are not authorized to access this content",
                    detail,
                }),
            )
                .into_response(),
        }
    }
}
