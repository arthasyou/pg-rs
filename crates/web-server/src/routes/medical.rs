use axum::{
    Router,
    routing::{get, post},
};
use toolcraft_axum_kit::{CommonError, CommonResponse};
use utoipa::OpenApi;

use crate::{
    dto::medical::{
        ListSelectableMetricsResponse, MetricSummaryDto, ObservationPointDto,
        QueryObservationParams, QueryRecipeObservationResponse, RecordObservationRequest,
        RecordObservationResponse, SelectableMetricDto, SourceInput, TaskStatusResponse,
        UploadMarkdownRequest, UploadMarkdownResponse, UploadMarkdownTaskResponse,
    },
    handlers::medical::{
        get_markdown_task, list_selectable_metrics, query_observations, record_observation,
        upload_markdown_data_source,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::medical::query_observations,
        crate::handlers::medical::record_observation,
        crate::handlers::medical::list_selectable_metrics,
        crate::handlers::medical::upload_markdown_data_source,
        crate::handlers::medical::get_markdown_task,
    ),
    components(
        schemas(
            QueryObservationParams,
            QueryRecipeObservationResponse,
            ObservationPointDto,
            MetricSummaryDto,
            RecordObservationRequest,
            SourceInput,
            RecordObservationResponse,
            ListSelectableMetricsResponse,
            SelectableMetricDto,
            UploadMarkdownRequest,
            UploadMarkdownResponse,
            UploadMarkdownTaskResponse,
            TaskStatusResponse,
            CommonResponse<QueryRecipeObservationResponse>,
            CommonResponse<RecordObservationResponse>,
            CommonResponse<ListSelectableMetricsResponse>,
            CommonResponse<UploadMarkdownResponse>,
            CommonResponse<UploadMarkdownTaskResponse>,
            CommonResponse<TaskStatusResponse>,
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
        .route(
            "/observations",
            get(query_observations).post(record_observation),
        )
        .route("/metrics/selectable", get(list_selectable_metrics))
        .route("/data-source/markdown", post(upload_markdown_data_source))
        .route(
            "/data-source/markdown/tasks/{task_id}",
            get(get_markdown_task),
        )
}
