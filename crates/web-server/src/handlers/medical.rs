use axum::extract::Query;
use demo_db::api::medical::HealthApi;
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};

use crate::{
    dto::medical::{QueryObservationRequest, QueryObservationResponse},
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
        .map_err(|e| Error::Custom(e.to_string()))?;

    let resp: QueryObservationResponse = (subject_id, result).into();

    // 4. 返回成功结果
    Ok(resp.into_common_response().to_json())
}
