use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Recipe 完整输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub output_metric_id: i64,
    pub deps: serde_json::Value,
    pub calc_key: String,
    pub arg_map: Option<serde_json::Value>,
    pub expr: Option<serde_json::Value>,
    pub created_at: OffsetDateTime,
}

/// 创建 Recipe 的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecipe {
    pub output_metric_id: i64,
    pub deps: serde_json::Value,
    pub calc_key: String,
    pub arg_map: Option<serde_json::Value>,
    pub expr: Option<serde_json::Value>,
}

/// 查询 Recipe 的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRecipe {
    pub output_metric_id: Option<i64>,
    pub calc_key: Option<String>,
}
