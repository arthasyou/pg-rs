use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateItemRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ItemResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
