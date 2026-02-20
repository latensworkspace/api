use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::{errors::error::ApiError, services::jwt::verify_jwt};

pub async fn jwt(mut req: Request<Body>, next: Next) -> Result<Response, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let None = auth_header {
        return Err(ApiError::Unauthorized);
    }

    let claims = verify_jwt(&auth_header.unwrap()).map_err(|_| ApiError::Unauthorized)?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
