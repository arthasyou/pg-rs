use serde::{Deserialize, Serialize};
use time::OffsetDateTime;



/// Recipe 的强类型 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId(pub i64);

impl From<i64> for RecipeId {
    fn from(value: i64) -> Self {
        RecipeId(value)
    }
}

impl From<RecipeId> for i64 {
    fn from(id: RecipeId) -> Self {
        id.0
    }
}

/// Recipe 完整输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub metric_id: i64,
    pub deps: serde_json::Value,
    pub calc_key: String,
    pub arg_map: serde_json::Value,
    pub expr: serde_json::Value,
    pub created_at: OffsetDateTime,
}

/// 创建 Recipe 的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecipe {
    pub metric_id: i64,
    pub deps: serde_json::Value,
    pub calc_key: String,
    pub arg_map: serde_json::Value,
    pub expr: serde_json::Value,
}

/// 查询 Recipe 的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRecipe {
    pub calc_key: Option<String>,
}
