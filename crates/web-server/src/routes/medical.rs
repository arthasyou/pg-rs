use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Router, routing::get};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::{
        example::{CreateItemRequest, ItemResponse, MessageResponse},
        medical::{RecordObservationRequest, RecordObservationResponse, SourceInput},
    },
    handlers::{
        example::ItemStore,
        medical::{query_observations, record_observation},
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
        crate::handlers::medical::record_observation,
    ),
    components(
        schemas(
            CreateItemRequest,
            ItemResponse,
            MessageResponse,
            RecordObservationRequest,
            RecordObservationResponse,
            SourceInput,
            CommonResponse<ItemResponse>,
            CommonResponse<Vec<ItemResponse>>,
            CommonResponse<MessageResponse>,
            CommonResponse<RecordObservationResponse>,
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
        .route("/observations", get(query_observations).post(record_observation))
        .with_state(store)
}
