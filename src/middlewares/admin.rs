use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{errors::error::ApiError, services::jwt::Claims};

pub async fn admin(req: Request<axum::body::Body>, next: Next) -> Response {
    let claims = match req.extensions().get::<Claims>() {
        Some(c) => c,
        None => {
            return ApiError::MissingToken.into_response();
        }
    };

    let admin_id = match std::env::var("ADMIN_USER_ID") {
        Ok(v) => v,
        Err(_) => return ApiError::Internal.into_response(),
    };

    if claims.sub != admin_id {
        return ApiError::Unauthorized.into_response();
    }

    next.run(req).await
}
