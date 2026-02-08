use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserRequest {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(email)]
    pub email: String,
}