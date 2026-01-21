use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::table::metric::dto::{MetricCode, MetricStatus, MetricValueType, MetricVisualization};

/// Recipe 类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecipeKind {
    Primitive,
    Derived,
}

impl RecipeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecipeKind::Primitive => "primitive",
            RecipeKind::Derived => "derived",
        }
    }
}

impl core::fmt::Display for RecipeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for RecipeKind {
    fn from(value: &str) -> Self {
        match value {
            "primitive" => RecipeKind::Primitive,
            "derived" => RecipeKind::Derived,
            other => {
                let normalized = other.trim().to_ascii_lowercase();
                match normalized.as_str() {
                    "primitive" => RecipeKind::Primitive,
                    "derived" => RecipeKind::Derived,
                    _ => RecipeKind::Primitive,
                }
            }
        }
    }
}

impl From<String> for RecipeKind {
    fn from(value: String) -> Self {
        RecipeKind::from(value.as_str())
    }
}

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
    pub kind: RecipeKind,
    pub deps: serde_json::Value,
    pub calc_key: Option<String>,
    pub arg_map: Option<serde_json::Value>,
    pub expr: Option<serde_json::Value>,
    pub metric_code: MetricCode,
    pub metric_name: String,
    pub unit: String,
    pub value_type: MetricValueType,
    pub visualization: MetricVisualization,
    pub status: MetricStatus,
    pub created_at: OffsetDateTime,
}

/// 创建 Recipe 的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecipe {
    pub kind: RecipeKind,
    pub deps: serde_json::Value,
    pub calc_key: Option<String>,
    pub arg_map: Option<serde_json::Value>,
    pub expr: Option<serde_json::Value>,
    pub metric_code: MetricCode,
    pub metric_name: String,
    pub unit: String,
    pub value_type: MetricValueType,
    pub visualization: MetricVisualization,
    pub status: MetricStatus,
}

/// 查询 Recipe 的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRecipe {
    pub kind: Option<RecipeKind>,
    pub calc_key: Option<String>,
}

/// Recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeSummary {
    pub id: RecipeId,
    pub metric_code: MetricCode,
    pub metric_name: String,
    pub unit: String,
    pub value_type: MetricValueType,
    pub visualization: MetricVisualization,
}

impl From<Recipe> for RecipeSummary {
    fn from(recipe: Recipe) -> Self {
        Self {
            id: recipe.id.into(),
            metric_code: recipe.metric_code,
            metric_name: recipe.metric_name,
            unit: recipe.unit,
            value_type: recipe.value_type,
            visualization: recipe.visualization,
        }
    }
}
