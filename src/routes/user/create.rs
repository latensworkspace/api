use axum::{extract::State, Json, http::StatusCode};
use crate::db::state::AppState;
use crate::models::user::user::CreateUserRequest;

pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserRequest>), StatusCode> {
    
    println!("{:?}", payload);

    let collection = state.db.collection::<CreateUserRequest>("users");

    match collection.insert_one(&payload).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(payload))),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}