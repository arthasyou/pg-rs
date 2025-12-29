use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Router, routing::get};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    handlers::example::{ItemStore, create_item, delete_item, get_item, health_check, list_items},
    models::example::{CreateItemRequest, ItemResponse, MessageResponse},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::example::list_items,
        crate::handlers::example::get_item,
        crate::handlers::example::create_item,
        crate::handlers::example::delete_item,
        crate::handlers::example::health_check,
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
        (name = "Example", description = "Example API endpoints")
    )
)]
pub struct ExampleApi;

pub fn example_routes() -> Router {
    let store: ItemStore = Arc::new(Mutex::new(HashMap::new()));

    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/{id}", get(get_item).delete(delete_item))
        .route("/health", get(health_check))
        .with_state(store)
}
