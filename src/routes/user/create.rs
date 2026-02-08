use axum::{Json, extract::State};
use validator::Validate;

use crate::{db::state::AppState, errors::error::ApiError, models::user::create::CreateUserRequest};

pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserRequest>, ApiError> {
    payload.validate().map_err(ApiError::from_validation)?;

    let collection = state.db.collection::<CreateUserRequest>("users");

    collection
        .insert_one(&payload)
        .await
        .map_err(|_| ApiError::Database)?;

    Ok(Json(payload))
}
