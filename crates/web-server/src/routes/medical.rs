use axum::{Router, routing::{get, post}};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::medical::{
        ListSelectableRecipesResponse, ObservationPointDto, QueryObservationRequest,
        QueryObservationResponse, RecordObservationRequest, RecordObservationResponse,
        RecipeDto, SelectableRecipeDto, SourceInput, UploadMarkdownRequest, UploadMarkdownResponse,
        ExtractHealthMetricsRequest, ExtractHealthMetricsResponse, HealthMetric, ExtractedHealthData,
    },
    handlers::medical::{list_selectable_recipes, query_observations, record_observation, upload_markdown_data_source, extract_health_metrics},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
        crate::handlers::medical::record_observation,
        crate::handlers::medical::list_selectable_recipes,
        crate::handlers::medical::upload_markdown_data_source,
        crate::handlers::medical::extract_health_metrics,
    ),
    components(
        schemas(
            QueryObservationRequest,
            QueryObservationResponse,
            RecipeDto,
            ObservationPointDto,
            RecordObservationRequest,
            SourceInput,
            RecordObservationResponse,
            ListSelectableRecipesResponse,
            SelectableRecipeDto,
            UploadMarkdownRequest,
            UploadMarkdownResponse,
            ExtractHealthMetricsRequest,
            ExtractHealthMetricsResponse,
            HealthMetric,
            ExtractedHealthData,
            CommonResponse<QueryObservationResponse>,
            CommonResponse<RecordObservationResponse>,
            CommonResponse<ListSelectableRecipesResponse>,
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
        .route("/recipes/selectable", get(list_selectable_recipes))
        .route("/data-source/markdown", post(upload_markdown_data_source))
        .route("/extract-metrics", post(extract_health_metrics))
}
