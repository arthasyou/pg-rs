use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Router, routing::get};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::example::{CreateItemRequest, ItemResponse, MessageResponse},
    handlers::{example::{ItemStore}, medical::query_observations},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
      
    ),
    components(
        schemas(
            CreateItemRequest,
            ItemResponse,
            MessageResponse,
            CommonResponse<ItemResponse>,
            CommonResponse<Vec<ItemResponse>>,
            CommonResponse<MessageResponse>,
            CommonError
        )
    ),
    tags(
        (name = "Medical", description = "Medical API endpoints")
    )
)]
pub struct MedicalApi;

pub fn medical_routes() -> Router {
    let store: ItemStore = Arc::new(Mutex::new(HashMap::new()));

    Router::new()
        .route("/observations", get(query_observations))
        .with_state(store)
}
