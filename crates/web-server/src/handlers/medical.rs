use axum::{Json, extract::Query};
use demo_db::{
    CreateDataSource, api::medical::HealthApi, dto::medical::RecordObservationWithSourceRequest,
};
use pg_tables::table::metric::dto::MetricKind;
use std::collections::HashSet;
use time::OffsetDateTime;
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};

use crate::{
    dto::medical::{
        ListSelectableMetricsResponse, QueryObservationParams, QueryRecipeObservationResponse,
        MetricSummaryDto, RecordObservationRequest, RecordObservationResponse, SelectableMetricDto,
        UploadMarkdownRequest, UploadMarkdownResponse, ObservationPointDto, format_rfc3339_utc,
    },
    error::Error,
    statics::db_manager::get_default_ctx,
    utils::parse_markdown,
};

#[utoipa::path(
    get,
    path = "/observations",
    tag = "Medical",
    params(
        QueryObservationParams
    ),
    responses(
        (status = 200, description = "Query observations", body = CommonResponse<QueryRecipeObservationResponse>),
    )
)]
pub async fn query_observations(
    Query(req): Query<QueryObservationParams>,
) -> ResponseResult<QueryRecipeObservationResponse> {
    // 1. 构造 HealthApi（轻量）
    let api = HealthApi::new(get_default_ctx());

    let subject_id = req.subject_id;

    // 2. web request → internal 参数
    let (query, range) = req.to_internal()?;

    // 3. 调用内部查询
    let result = api
        .query_observation(query, range)
        .await
        .map_err(Error::Core)?;

    let resp = QueryRecipeObservationResponse {
        subject_id,
        metric: MetricSummaryDto {
            id: result.metric.id,
            metric_code: result.metric.metric_code.0,
            metric_name: result.metric.metric_name,
            unit: result.metric.unit,
            value_type: result.metric.value_type.to_string(),
            visualization: result.metric.visualization.to_string(),
        },
        points: result
            .points
            .into_iter()
            .map(|p| ObservationPointDto {
                value: p.value.as_str().to_string(),
                value_num: p.value.as_str().parse::<f64>().ok(),
                observed_at: format_rfc3339_utc(p.observed_at),
            })
            .collect(),
    };

    Ok(resp.into_common_response().to_json())
}

#[utoipa::path(
    post,
    path = "/observations",
    tag = "Medical",
    request_body = RecordObservationRequest,
    responses(
        (status = 200, description = "Record observation", body = CommonResponse<RecordObservationResponse>),
    )
)]
pub async fn record_observation(
    Json(req): Json<RecordObservationRequest>,
) -> ResponseResult<RecordObservationResponse> {
    let api = HealthApi::new(get_default_ctx());

    // web request → internal 参数
    let (subject_id, metric_id, value, observed_at, source) = req.to_internal()?;

    // 构造业务请求
    let internal_req = RecordObservationWithSourceRequest {
        subject_id,
        metric_id,
        value,
        observed_at,
        source,
    };

    // 调用业务 API
    let result = api
        .record_observation_with_source(internal_req)
        .await
        .map_err(Error::Core)?;

    let resp = RecordObservationResponse {
        observation_id: result.observation_id.0,
        source_id: result.source_id.0,
    };

    Ok(resp.into_common_response().to_json())
}

#[utoipa::path(
    get,
    path = "/metrics/selectable",
    tag = "Medical",
    responses(
        (status = 200, description = "List selectable metrics", body = CommonResponse<ListSelectableMetricsResponse>),
    )
)]
pub async fn list_selectable_metrics() -> ResponseResult<ListSelectableMetricsResponse> {
    let api = HealthApi::new(get_default_ctx());

    let metrics = api.list_selectable_metrics().await.map_err(Error::Core)?;

    let resp = ListSelectableMetricsResponse {
        metrics: metrics
            .into_iter()
            .map(|m| SelectableMetricDto {
                id: m.id.0,
                name: m.name,
                unit: m.unit,
            })
            .collect(),
    };

    Ok(resp.into_common_response().to_json())
}

#[utoipa::path(
    post,
    path = "/data-source/markdown",
    tag = "Medical",
    request_body = UploadMarkdownRequest,
    responses(
        (status = 200, description = "Upload and parse markdown file", body = CommonResponse<UploadMarkdownResponse>),
    )
)]
pub async fn upload_markdown_data_source(
    Json(req): Json<UploadMarkdownRequest>,
) -> ResponseResult<UploadMarkdownResponse> {
    use demo_db::DataSourceKind;
    use pg_tables::table::data_source::service::DataSourceService;

    if req.file_content.is_empty() {
        return Err(Error::Custom("No file content provided".to_string()))?;
    }

    // 解析 Markdown
    let parsed_data = parse_markdown(&req.file_content);

    // 准备数据源数据
    let ctx = get_default_ctx();
    let service = DataSourceService::new(ctx.clone());

    let input = CreateDataSource {
        kind: DataSourceKind::from(req.source_type.as_str()),
        name: req.source_name.clone(),
        metadata: Some(parsed_data.clone()),
    };

    // 插入数据库
    let model = service.create(input).await.map_err(|e: demo_db::Error| {
        Error::Custom(format!("Failed to insert data source: {}", e))
    })?;

    let api = HealthApi::new(ctx);
    let metrics = api.list_selectable_metrics().await.map_err(Error::Core)?;
    let extracted = extract_metric_values(&metrics, &req.file_content);
    let mut records_inserted = 0;

    for (metric_id, value) in extracted {
        let result = api
            .record_observation_with_source_id(
                demo_db::SubjectId(req.subject_id),
                metric_id,
                demo_db::ObservationValue(value),
                OffsetDateTime::now_utc(),
                model.id,
            )
            .await;

        if result.is_ok() {
            records_inserted += 1;
        }
    }

    let now = OffsetDateTime::now_utc();
    let created_at = now
        .to_offset(time::UtcOffset::UTC)
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "invalid datetime".to_string());

    let resp = UploadMarkdownResponse {
        source_id: model.id.0,
        source_type: model.kind.to_string(),
        source_name: model.name,
        parsed_data,
        created_at,
        records_inserted,
    };

    Ok(resp.into_common_response().to_json())
}

fn extract_metric_values(
    metrics: &[pg_tables::table::metric::dto::Metric],
    content: &str,
) -> Vec<(pg_tables::table::metric::dto::MetricId, String)> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    for metric in metrics {
        if metric.kind != MetricKind::Primitive || seen.contains(&metric.id) {
            continue;
        }

        let code = metric.code.as_ref().to_ascii_lowercase();
        let name = metric.name.to_ascii_lowercase();

        for line in content.lines() {
            let line_trim = line.trim();
            if line_trim.is_empty() {
                continue;
            }
            let line_lower = line_trim.to_ascii_lowercase();
            if let Some(value) = extract_value_from_line(line_trim, &line_lower, &code, &name) {
                if !value.is_empty() {
                    results.push((metric.id, value));
                    seen.insert(metric.id);
                    break;
                }
            }
        }
    }

    results
}

fn extract_value_from_line(
    line: &str,
    line_lower: &str,
    code: &str,
    name: &str,
) -> Option<String> {
    let key = if !code.is_empty() && line_lower.contains(code) {
        code
    } else if !name.is_empty() && line_lower.contains(name) {
        name
    } else {
        return None;
    };

    let pos = line_lower.find(key)?;
    let mut after = line[pos + key.len()..].trim();
    after = after.trim_start_matches(|c: char| {
        matches!(c, ':' | '：' | '-' | '—' | '–' | '|' | ' ' | '\t')
    });

    let value = match after.split_once('|') {
        Some((v, _)) => v.trim(),
        None => after,
    };

    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}
