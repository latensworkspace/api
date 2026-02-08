use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub id: String,
    pub name: String,
    pub email: String,
}