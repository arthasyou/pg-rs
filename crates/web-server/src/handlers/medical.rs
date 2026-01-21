use axum::{Json, extract::Query};
use demo_db::{
    CreateDataSource, api::medical::HealthApi, dto::medical::RecordObservationWithSourceRequest,
};
use time::OffsetDateTime;
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};

use crate::{
    dto::medical::{
        ExtractHealthMetricsRequest, ExtractHealthMetricsResponse, ExtractedHealthData,
        HealthMetric, ListSelectableMetricsResponse, QueryObservationRequest,
        QueryObservationResponse, QueryRecipeObservationRequest, QueryRecipeObservationResponse,
        RecipeSummaryDto, RecordObservationRequest, RecordObservationResponse, SelectableMetricDto,
        UploadMarkdownRequest, UploadMarkdownResponse, ObservationPointDto, format_rfc3339_utc,
    },
    error::Error,
    statics::db_manager::get_default_ctx,
    utils::{extract_health_metrics_with_llm, parse_markdown},
};

#[utoipa::path(
    get,
    path = "/observations",
    tag = "Medical",
    params(
        QueryObservationRequest
    ),
    responses(
        (status = 200, description = "Query observations", body = CommonResponse<QueryObservationResponse>),
    )
)]
pub async fn query_observations(
    Query(req): Query<QueryObservationRequest>,
) -> ResponseResult<QueryObservationResponse> {
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

    let resp: QueryObservationResponse = (subject_id, result).into();

    // 4. 返回成功结果
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
    path = "/recipes/observations",
    tag = "Medical",
    params(
        QueryRecipeObservationRequest
    ),
    responses(
        (status = 200, description = "Query composite metric observations", body = CommonResponse<QueryRecipeObservationResponse>),
    )
)]
pub async fn query_recipe_observations(
    Query(req): Query<QueryRecipeObservationRequest>,
) -> ResponseResult<QueryRecipeObservationResponse> {
    let api = HealthApi::new(get_default_ctx());

    let subject_id = req.subject_id;

    let (query, range) = req.to_internal()?;

    let result = api
        .query_composite_metric(query, range)
        .await
        .map_err(Error::Core)?;

    let resp = QueryRecipeObservationResponse {
        subject_id,
        metric: RecipeSummaryDto {
            id: result.metric.id.0,
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
    let service = DataSourceService::new(ctx);

    let input = CreateDataSource {
        kind: DataSourceKind::from(req.source_type.as_str()),
        name: req.source_name.clone(),
        metadata: Some(parsed_data.clone()),
    };

    // 插入数据库
    let model = service.create(input).await.map_err(|e: demo_db::Error| {
        Error::Custom(format!("Failed to insert data source: {}", e))
    })?;

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
    };

    Ok(resp.into_common_response().to_json())
}

#[utoipa::path(
    post,
    path = "/extract-metrics",
    tag = "Medical",
    request_body = ExtractHealthMetricsRequest,
    responses(
        (status = 200, description = "Extract health metrics from document", body = CommonResponse<ExtractHealthMetricsResponse>),
    )
)]
pub async fn extract_health_metrics(
    Json(req): Json<ExtractHealthMetricsRequest>,
) -> ResponseResult<ExtractHealthMetricsResponse> {
    use demo_db::DataSourceKind;
    use pg_tables::table::data_source::service::DataSourceService;
    use serde_json::Value as JsonValue;

    // 获取 LLM 配置
    let config = crate::statics::llm_client::get_llm_config();

    // 使用 LLM 提取医疗指标
    let extracted_json =
        extract_health_metrics_with_llm(&req.content, &config.base_url, &config.model)
            .await
            .map_err(|e| Error::Custom(format!("Failed to extract metrics with LLM: {}", e)))?;

    // 解析提取的数据
    let patient_info = extracted_json
        .get("patient_info")
        .cloned()
        .unwrap_or_else(|| JsonValue::Object(Default::default()));

    let metrics: Vec<HealthMetric> = extracted_json
        .get("metrics")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| serde_json::from_value(item.clone()).ok())
                .collect()
        })
        .unwrap_or_default();

    let diagnoses: Vec<String> = extracted_json
        .get("diagnoses")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let recommendations: Vec<String> = extracted_json
        .get("recommendations")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let extracted_data = ExtractedHealthData {
        patient_info,
        metrics: metrics.clone(),
        diagnoses,
        recommendations,
    };

    // 保存数据来源到数据库
    let ctx = get_default_ctx();
    let service = DataSourceService::new(ctx.clone());

    let source_type = req
        .source_type
        .clone()
        .unwrap_or_else(|| "import".to_string());
    let source_name = req
        .source_name
        .clone()
        .unwrap_or_else(|| "Health Metrics Extraction".to_string());

    let input = CreateDataSource {
        kind: DataSourceKind::from(source_type.as_str()),
        name: source_name.clone(),
        metadata: Some(extracted_json.clone()),
    };

    let source_model = service.create(input).await.map_err(|e: demo_db::Error| {
        Error::Custom(format!("Failed to insert data source: {}", e))
    })?;

    let source_id = source_model.id.0;

    // 插入观测记录
    let api = HealthApi::new(ctx);
    let mut records_inserted = 0;

    // 将提取的指标插入到数据库
    for metric in &metrics {
        // 尝试匹配指标代码到数据库中的指标
        if let Ok(metrics_list) = api.list_selectable_metrics().await {
            for db_metric in metrics_list {
                let metric_lower = metric.metric_code.to_lowercase();
                let db_metric_lower = db_metric.name.to_lowercase();

                if metric_lower.contains(&db_metric_lower)
                    || db_metric_lower.contains(&metric_lower)
                {
                    let obs_req = RecordObservationWithSourceRequest {
                        subject_id: demo_db::SubjectId(req.subject_id),
                        metric_id: db_metric.id,
                        value: demo_db::ObservationValue(metric.value.clone()),
                        observed_at: OffsetDateTime::now_utc(),
                        source: demo_db::CreateDataSource {
                            kind: demo_db::DataSourceKind::Import,
                            name: source_name.clone(),
                            metadata: Some(
                                serde_json::to_value(metric)
                                    .unwrap_or_else(|_| JsonValue::Object(Default::default())),
                            ),
                        },
                    };

                    if api.record_observation_with_source(obs_req).await.is_ok() {
                        records_inserted += 1;
                        break;
                    }
                }
            }
        }
    }

    let resp = ExtractHealthMetricsResponse {
        data: extracted_data,
        source_id: Some(source_id),
        records_inserted,
    };

    Ok(resp.into_common_response().to_json())
}
