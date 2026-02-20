use axum::{Json, extract::{Request, State}, response::IntoResponse};

use crate::{db::state::AppState, errors::error::ApiError, models::user::get::GetUserRequest, services::jwt::Claims};
use mongodb::bson::doc;

pub async fn handler(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Result<Json<GetUserRequest>, ApiError> {
    let collection = state.db.collection::<mongodb::bson::Document>("users");

    let claims = match req.extensions().get::<Claims>() {
        Some(c) => c,
        None => {
            return ApiError::MissingToken;
        }
    };

    let user = collection
        .find_one(doc! { "_id": claims.sub })
        .await
        .map_err(|_| ApiError::Database)?
        .ok_or(ApiError::UserNotFound)?;

    Ok(Json(GetUserRequest {
        id: user
            .get_object_id("_id")
            .unwrap()
            .to_string(),
        name: user
            .get_str("name")
            .unwrap_or("")
            .to_string(),
        email: user
            .get_str("email")
            .unwrap_or("")
            .to_string(),
    }))
}