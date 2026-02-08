use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::state::AppState,
    errors::error::ApiError, models::user::get::GetUserRequest,
};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetUserRequest>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::UserNotFound)?;

    let collection = state.db.collection::<mongodb::bson::Document>("users");

    let user = collection
        .find_one(doc! { "_id": object_id })
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
