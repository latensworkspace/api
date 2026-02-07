use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use mongodb::bson::doc;

use crate::db::state::AppState;
use crate::models::user::user::CreateUserRequest;

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<CreateUserRequest>>, StatusCode> {
    let collection = state
        .db
        .collection::<CreateUserRequest>("users");

    let mut cursor = collection
        .find(doc! {})
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut users = Vec::new();

    while cursor
        .advance()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        users.push(
            cursor
                .deserialize_current()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        );
    }

    Ok(Json(users))
}
