use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use toolcraft_axum_kit::{CommonError, CommonResponse, IntoCommonResponse, ResponseResult};
use utoipa;

use crate::dto::example::{CreateItemRequest, ItemResponse, MessageResponse};

// 简单的内存存储，实际项目中应该使用数据库
pub type ItemStore = Arc<Mutex<HashMap<u64, ItemResponse>>>;

#[utoipa::path(
    get,
    path = "/example/items",
    tag = "Example",
    responses(
        (status = 200, description = "List all items", body = CommonResponse<Vec<ItemResponse>>),
        (status = 401, description = "Unauthorized", body = CommonError),
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn list_items(State(store): State<ItemStore>) -> ResponseResult<Vec<ItemResponse>> {
    let items = store.lock().unwrap();
    let items_vec: Vec<ItemResponse> = items.values().cloned().collect();
    Ok(items_vec.into_common_response().to_json())
}

#[utoipa::path(
    get,
    path = "/example/items/{id}",
    tag = "Example",
    params(
        ("id" = u64, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Item found", body = CommonResponse<ItemResponse>),
        (status = 404, description = "Item not found", body = CommonError),
        (status = 401, description = "Unauthorized", body = CommonError),
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_item(
    Path(id): Path<u64>,
    State(store): State<ItemStore>,
) -> ResponseResult<ItemResponse> {
    let items = store.lock().unwrap();
    if let Some(item) = items.get(&id) {
        Ok(item.clone().into_common_response().to_json())
    } else {
        Err((
            StatusCode::NOT_FOUND,
            CommonError {
                code: 404,
                message: format!("Item with id {id} not found"),
            }
            .to_json(),
        ))
    }
}

#[utoipa::path(
    post,
    path = "/example/items",
    tag = "Example",
    request_body = CreateItemRequest,
    responses(
        (status = 200, description = "Item created", body = CommonResponse<ItemResponse>),
        (status = 400, description = "Invalid request", body = CommonError),
        (status = 401, description = "Unauthorized", body = CommonError),
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_item(
    State(store): State<ItemStore>,
    axum::Json(payload): axum::Json<CreateItemRequest>,
) -> ResponseResult<ItemResponse> {
    // 验证请求
    use validator::Validate;
    if let Err(e) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            CommonError {
                code: 400,
                message: format!("Validation error: {e}"),
            }
            .to_json(),
        ));
    }

    let mut items = store.lock().unwrap();
    let id = items.len() as u64 + 1;

    let item = ItemResponse {
        id,
        name: payload.name,
        description: payload.description,
        created_at: "now".to_string(),
        // created_at: OffsetDateTime::now_utc().to_rfc3339(),
    };

    items.insert(id, item.clone());

    Ok(item.into_common_response().to_json())
}

#[utoipa::path(
    delete,
    path = "/example/items/{id}",
    tag = "Example",
    params(
        ("id" = u64, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Item deleted", body = CommonResponse<MessageResponse>),
        (status = 404, description = "Item not found", body = CommonError),
        (status = 401, description = "Unauthorized", body = CommonError),
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_item(
    Path(id): Path<u64>,
    State(store): State<ItemStore>,
) -> ResponseResult<MessageResponse> {
    let mut items = store.lock().unwrap();
    if items.remove(&id).is_some() {
        Ok(MessageResponse {
            message: format!("Item {id} deleted successfully"),
        }
        .into_common_response()
        .to_json())
    } else {
        Err((
            StatusCode::NOT_FOUND,
            CommonError {
                code: 404,
                message: format!("Item with id {id} not found"),
            }
            .to_json(),
        ))
    }
}

#[utoipa::path(
    get,
    path = "/example/health",
    tag = "Example",
    responses(
        (status = 200, description = "Service is healthy", body = CommonResponse<MessageResponse>),
    )
)]
pub async fn health_check() -> ResponseResult<MessageResponse> {
    Ok(MessageResponse {
        message: "Service is healthy".to_string(),
    }
    .into_common_response()
    .to_json())
}
