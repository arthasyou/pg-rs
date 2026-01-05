use axum::{Router, routing::get};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::medical::{
        MetricDto, ObservationPointDto, QueryObservationRequest, QueryObservationResponse,
        RecordObservationRequest, RecordObservationResponse, SourceInput,
    },
    handlers::medical::{query_observations, record_observation},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
        crate::handlers::medical::record_observation,
    ),
    components(
        schemas(
            QueryObservationRequest,
            QueryObservationResponse,
            MetricDto,
            ObservationPointDto,
            RecordObservationRequest,
            SourceInput,
            RecordObservationResponse,
            CommonResponse<QueryObservationResponse>,
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
    Router::new()
        .route("/observations", get(query_observations).post(record_observation))
}
