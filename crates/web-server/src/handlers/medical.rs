use axum::{Json, extract::{Path, Query}};
use demo_db::{
    CreateDataSource, api::medical::HealthApi, dto::medical::RecordObservationWithSourceRequest,
};
use pg_tables::table::metric::dto::{MetricKind, MetricValueType};
use std::collections::HashSet;
use time::OffsetDateTime;
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};

use crate::{
    dto::medical::{
        ListSelectableMetricsResponse, QueryObservationParams, QueryRecipeObservationResponse,
        MetricSummaryDto, RecordObservationRequest, RecordObservationResponse, SelectableMetricDto,
        TaskStatusResponse, UploadMarkdownRequest, UploadMarkdownResponse,
        UploadMarkdownTaskResponse, ObservationPointDto, format_rfc3339_utc,
    },
    error::Error,
    statics::db_manager::get_default_ctx,
    statics::task_store,
};
use serde_json::json;

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
) -> ResponseResult<UploadMarkdownTaskResponse> {
    use demo_db::DataSourceKind;
    use pg_tables::table::data_source::service::DataSourceService;

    if req.file_content.is_empty() {
        return Err(Error::Custom("No file content provided".to_string()))?;
    }

    let task_id = task_store::create_task();
    let req_task = req.clone();
    let task_id_clone = task_id.clone();

    tokio::spawn(async move {
        task_store::set_running(&task_id_clone);

        let result: Result<UploadMarkdownResponse, Error> = async {
            let ctx = get_default_ctx();
            let service = DataSourceService::new(ctx.clone());

            let observed_at = parse_report_date(&req_task.file_content)?;
            let observed_at_str = format_rfc3339_utc(observed_at);

            let api = HealthApi::new(ctx);
            let metrics = api.list_selectable_metrics().await.map_err(Error::Core)?;
            let extracted = extract_metric_values(&metrics, &req_task.file_content);
            let mut records_inserted = 0;
            let metrics_json = extracted
                .iter()
                .map(|(metric_id, value)| {
                    json!({
                        "metric_id": metric_id.0,
                        "value": value,
                    })
                })
                .collect::<Vec<_>>();

            let parsed_data = json!({
                "observed_at": observed_at_str,
                "metrics": metrics_json,
            });

            let input = CreateDataSource {
                kind: DataSourceKind::from(req_task.source_type.as_str()),
                name: req_task.source_name.clone(),
                metadata: Some(parsed_data.clone()),
            };

            let model = service.create(input).await.map_err(|e: demo_db::Error| {
                Error::Custom(format!("Failed to insert data source: {}", e))
            })?;

            for (metric_id, value) in extracted {
                let result = api
                    .record_observation_with_source_id(
                        demo_db::SubjectId(req_task.subject_id),
                        metric_id,
                        demo_db::ObservationValue(value),
                        observed_at,
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

            Ok(UploadMarkdownResponse {
                source_id: model.id.0,
                source_type: model.kind.to_string(),
                source_name: model.name,
                parsed_data,
                created_at,
                records_inserted,
            })
        }
        .await;

        match result {
            Ok(resp) => task_store::set_result(&task_id_clone, resp),
            Err(err) => task_store::set_error(&task_id_clone, err.to_string()),
        }
    });

    Ok(UploadMarkdownTaskResponse { task_id }
        .into_common_response()
        .to_json())
}

#[utoipa::path(
    get,
    path = "/data-source/markdown/tasks/{task_id}",
    tag = "Medical",
    params(
        ("task_id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Get markdown task status", body = CommonResponse<TaskStatusResponse>),
    )
)]
pub async fn get_markdown_task(
    Path(task_id): Path<String>,
) -> ResponseResult<TaskStatusResponse> {
    let state = task_store::get_task(&task_id)
        .ok_or_else(|| Error::Core(pg_core::Error::not_found("task", &task_id)))?;

    let resp = TaskStatusResponse {
        task_id,
        status: state.status.as_str().to_string(),
        result: state.result,
        error: state.error,
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

        let keys = build_metric_keys(metric);

        for line in content.lines() {
            let line_trim = line.trim();
            if line_trim.is_empty() {
                continue;
            }
            let line_lower = line_trim.to_ascii_lowercase();
            let mut matched = false;
            for key in &keys {
                if let Some(value) = extract_value_from_line(
                    line_trim,
                    &line_lower,
                    key,
                    &metric.value_type,
                ) {
                    if !value.is_empty() {
                        results.push((metric.id, value));
                        seen.insert(metric.id);
                        matched = true;
                        break;
                    }
                }
            }
            if matched {
                break;
            }
        }
    }

    results
}

fn build_metric_keys(metric: &pg_tables::table::metric::dto::Metric) -> Vec<String> {
    let mut keys = vec![metric.name.to_ascii_lowercase()];
    match metric.name.as_str() {
        "尿比重" => keys.push("比重".to_string()),
        "尿PH值" => keys.push("ph值".to_string()),
        "尿蛋白" => keys.push("蛋白质".to_string()),
        "尿糖" => keys.push("葡萄糖".to_string()),
        "幽门螺杆菌抗体" => {
            keys.push("幽门螺旋杆菌抗体".to_string());
            keys.push("幽门螺旋杆菌抗体检测".to_string());
        }
        _ => {}
    }
    let mut seen = HashSet::new();
    keys.into_iter()
        .filter(|key| seen.insert(key.clone()))
        .collect()
}

fn extract_value_from_line(
    line: &str,
    line_lower: &str,
    key: &str,
    value_type: &MetricValueType,
) -> Option<String> {
    if key.is_empty() || !line_lower.contains(key) {
        return None;
    }

    let pos = line_lower.find(key)?;
    let mut after = line[pos + key.len()..].trim();
    after = after.trim_start_matches(|c: char| {
        matches!(c, ':' | '：' | '-' | '—' | '–' | '|' | ' ' | '\t')
    });

    let raw_value = match after.split_once('|') {
        Some((v, _)) => v.trim(),
        None => after,
    };

    match value_type {
        MetricValueType::Integer | MetricValueType::Float | MetricValueType::Decimal => {
            return first_number_in_string(raw_value);
        }
        MetricValueType::Boolean | MetricValueType::Text => {
            return extract_text_value(raw_value);
        }
    }
}

fn first_number_in_string(value: &str) -> Option<String> {
    let mut start: Option<usize> = None;
    let mut end: usize = 0;

    for (idx, ch) in value.char_indices() {
        if start.is_none() {
            if ch.is_ascii_digit() {
                start = Some(idx);
                end = idx + ch.len_utf8();
            }
            continue;
        }

        if ch.is_ascii_digit() || ch == '.' {
            end = idx + ch.len_utf8();
        } else {
            break;
        }
    }

    start.map(|s| value[s..end].to_string())
}

fn extract_text_value(value: &str) -> Option<String> {
    if let Some(preferred) = preferred_text_value(value) {
        return Some(preferred);
    }

    for token in value.split_whitespace() {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let stripped = token
            .trim_matches(|c: char| matches!(c, '(' | ')' | '（' | '）'))
            .trim();
        if stripped.is_empty() {
            continue;
        }
        if is_placeholder_value(stripped) {
            continue;
        }
        if !stripped.chars().any(is_cjk_char) {
            continue;
        }
        return Some(stripped.to_string());
    }
    None
}

fn is_placeholder_value(value: &str) -> bool {
    if matches!(value, "-" | "—" | "/" | "NA" | "N/A" | "检测") {
        return true;
    }
    value.chars().all(|c| c.is_ascii_uppercase()) && value.len() <= 6
}

fn is_cjk_char(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}')
}

fn preferred_text_value(value: &str) -> Option<String> {
    let candidates = ["阴性", "阳性", "弱阳性", "强阳性", "未检出", "未发现", "正常", "异常"];
    for candidate in candidates {
        if value.contains(candidate) {
            return Some(candidate.to_string());
        }
    }
    None
}

fn parse_report_date(content: &str) -> Result<OffsetDateTime, Error> {
    for token in content.split_whitespace() {
        let trimmed = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '-');
        if trimmed.len() == 10
            && trimmed.as_bytes()[4] == b'-'
            && trimmed.as_bytes()[7] == b'-'
            && trimmed.chars().all(|c| c.is_ascii_digit() || c == '-')
        {
            let year: i32 = trimmed[0..4].parse().unwrap_or(0);
            let month: u8 = trimmed[5..7].parse().unwrap_or(0);
            let day: u8 = trimmed[8..10].parse().unwrap_or(0);
            if let Ok(date) = time::Date::from_calendar_date(
                year,
                time::Month::try_from(month).unwrap_or(time::Month::January),
                day,
            ) {
                return Ok(OffsetDateTime::new_utc(date, time::Time::MIDNIGHT));
            }
        }
    }

    Err(Error::Custom(
        "report date not found in markdown content".to_string(),
    ))
}
