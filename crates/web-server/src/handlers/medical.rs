use axum::{extract::Query, http::StatusCode};
use demo_db::api::medical::HealthApi;
use toolcraft_axum_kit::{CommonError, CommonResponse, IntoCommonResponse, ResponseResult};

use crate::{
    dto::medical::{QueryObservationRequest, QueryObservationResponse},
    statics::db_manager::get_default_ctx,
};

// use crate::{
//     dto::observation::{QueryObservationRequest, QueryObservationResponse},
//     error::{Error, Result},
//     infra::get_default_ctx,
// }; // 你之前写的 ctx 获取函数

#[utoipa::path(
    get,
    path = "/health/observations",
    tag = "Health",
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
    let (query, range) = match req.to_internal() {
        Ok(v) => v,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CommonError {
                    code: 400,
                    message: "invalid request".to_string(),
                }
                .to_json(),
            ));
        }
    };

    // 3. 调用内部查询
    let result = match api.query_observation(query, range).await {
        Ok(v) => v,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CommonError {
                    code: 400,
                    message: e.to_string(),
                }
                .to_json(),
            ));
        }
    };

    let resp: QueryObservationResponse = (subject_id, result).into();

    // 4. 返回成功结果
    Ok(resp.into_common_response().to_json())
}
