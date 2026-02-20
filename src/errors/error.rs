use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum ApiError {
    Validation { errors: Vec<String> },
    Database,
    UserNotFound,
    Unauthorized,
    MissingToken,
    NotFound,
    Internal,
}

impl ApiError {
    pub fn from_validation(errors: ValidationErrors) -> Self {
        let fields: Vec<String> = errors
            .field_errors()
            .keys()
            .map(|field| field.to_string())
            .collect();

        ApiError::Validation { errors: fields }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Validation { errors } => {
                (StatusCode::BAD_REQUEST, Json(json!({ "errors": errors }))).into_response()
            }

            ApiError::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "user_not_found" })),
            )
                .into_response(),

            ApiError::Database => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error" })),
            )
                .into_response(),

            ApiError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "missing_jwt_token" })),
            )
                .into_response(),

            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "not_found" }))).into_response()
            }

            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "unauthorized" })),
            )
                .into_response(),

            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "internal_server_error" })),
            )
                .into_response(),
        }
    }
}
