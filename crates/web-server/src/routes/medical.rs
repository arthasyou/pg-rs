use axum::{Router, routing::{get, post}};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::medical::{
        ListSelectableMetricsResponse, ObservationPointDto, QueryObservationRequest,
        QueryObservationResponse, QueryRecipeObservationRequest, QueryRecipeObservationResponse,
        RecordObservationRequest, RecordObservationResponse, MetricDto, SelectableMetricDto,
        SourceInput, UploadMarkdownRequest, UploadMarkdownResponse, ExtractHealthMetricsRequest,
        ExtractHealthMetricsResponse, HealthMetric, ExtractedHealthData, MetricSummaryDto,
    },
    handlers::medical::{
        list_selectable_metrics, query_observations, query_recipe_observations, record_observation,
        upload_markdown_data_source, extract_health_metrics,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
        crate::handlers::medical::query_recipe_observations,
        crate::handlers::medical::record_observation,
        crate::handlers::medical::list_selectable_metrics,
        crate::handlers::medical::upload_markdown_data_source,
        crate::handlers::medical::extract_health_metrics,
    ),
    components(
        schemas(
            QueryObservationRequest,
            QueryObservationResponse,
            QueryRecipeObservationRequest,
            QueryRecipeObservationResponse,
            MetricDto,
            ObservationPointDto,
            MetricSummaryDto,
            RecordObservationRequest,
            SourceInput,
            RecordObservationResponse,
            ListSelectableMetricsResponse,
            SelectableMetricDto,
            UploadMarkdownRequest,
            UploadMarkdownResponse,
            ExtractHealthMetricsRequest,
            ExtractHealthMetricsResponse,
            HealthMetric,
            ExtractedHealthData,
            CommonResponse<QueryObservationResponse>,
            CommonResponse<RecordObservationResponse>,
            CommonResponse<ListSelectableMetricsResponse>,
            CommonResponse<UploadMarkdownResponse>,
            CommonResponse<ExtractHealthMetricsResponse>,
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
        .route("/recipes/observations", get(query_recipe_observations))
        .route("/metrics/selectable", get(list_selectable_metrics))
        .route("/data-source/markdown", post(upload_markdown_data_source))
        .route("/extract-metrics", post(extract_health_metrics))
}
