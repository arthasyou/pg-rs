use axum::extract::Query;
use axum::Json;
use demo_db::{
    api::medical::HealthApi,
    dto::medical::RecordObservationWithSourceRequest,
};
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};

use crate::{
    dto::medical::{
        QueryObservationRequest, QueryObservationResponse, RecordObservationRequest,
        RecordObservationResponse,
    },
    error::Error,
    statics::db_manager::get_default_ctx,
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
    let (query, range) = req
        .to_internal()
        .map_err(|_| Error::Custom("invalid request".into()))?;

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
