use axum::{Router, routing::get};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::medical::{
        ListSelectableMetricsResponse, MetricDto, ObservationPointDto, QueryObservationRequest,
        QueryObservationResponse, RecordObservationRequest, RecordObservationResponse,
        SelectableMetricDto, SourceInput,
    },
    handlers::medical::{list_selectable_metrics, query_observations, record_observation},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
        crate::handlers::medical::record_observation,
        crate::handlers::medical::list_selectable_metrics,
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
            ListSelectableMetricsResponse,
            SelectableMetricDto,
            CommonResponse<QueryObservationResponse>,
            CommonResponse<RecordObservationResponse>,
            CommonResponse<ListSelectableMetricsResponse>,
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
        .route("/metrics/selectable", get(list_selectable_metrics))
}
