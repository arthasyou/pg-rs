use demo_db::{
    CreateDataSource, DataSourceKind, ObservationValue, RecipeId, SubjectId,
    dto::{
        base::Range,
        medical::{ObservationQueryResult, QueryObservationSeries},
    },
};
use pg_tables::table::metric::dto::MetricValueType;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::{OffsetDateTime, UtcOffset};
use utoipa::{IntoParams, ToSchema};

use crate::error::{Error, Result};

fn parse_rfc3339(input: &str) -> Result<OffsetDateTime> {
    let dt = OffsetDateTime::parse(input.trim(), &time::format_description::well_known::Rfc3339)
        .map_err(|_| {
            Error::Custom(
                "invalid datetime format, expected RFC3339, e.g. '2025-12-30T10:02:43.893518Z'"
                    .into(),
            )
        })?;
    Ok(dt.to_offset(UtcOffset::UTC))
}

fn format_rfc3339_utc(dt: OffsetDateTime) -> String {
    dt.to_offset(UtcOffset::UTC)
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "invalid datetime".to_string())
}

// =========================
// List Selectable Recipes
// =========================

#[derive(Debug, Serialize, ToSchema)]
pub struct ListSelectableRecipesResponse {
    pub recipes: Vec<SelectableRecipeDto>,
}

/// 前端下拉框选项（仅保留必要字段）
#[derive(Debug, Serialize, ToSchema)]
pub struct SelectableRecipeDto {
    pub id: i64,
    pub name: Option<String>,
    pub unit: Option<String>,
}

// =========================
// Query Observation
// =========================

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct QueryObservationRequest {
    /// subject 全局 ID（web 层用裸 i64）
    pub subject_id: i64,

    /// recipe 全局 ID
    pub recipe_id: i64,

    /// 查询起始时间（RFC3339, 例如：2025-12-30T10:02:43.893518Z）
    pub start_at: Option<String>,

    /// 查询结束时间（RFC3339, 例如：2025-12-30T10:02:43.893518Z）
    pub end_at: Option<String>,
}

impl QueryObservationRequest {
    pub fn to_internal(self) -> Result<(QueryObservationSeries, Range<OffsetDateTime>)> {
        let start = match self.start_at {
            Some(ref s) => parse_rfc3339(s)?,
            None => OffsetDateTime::from_unix_timestamp(0).unwrap(),
        };

        let end = match self.end_at {
            Some(ref s) => parse_rfc3339(s)?,
            None => OffsetDateTime::now_utc(),
        };

        let range = Range {
            from: Some(start),
            to: Some(end),
        };

        let query = QueryObservationSeries {
            subject_id: self.subject_id.into(),
            recipe_id: self.recipe_id.into(),
        };

        Ok((query, range))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueryObservationResponse {
    pub subject_id: i64,
    pub recipe: RecipeDto,
    pub points: Vec<ObservationPointDto>,
}

impl From<(i64, ObservationQueryResult)> for QueryObservationResponse {
    fn from((subject_id, v): (i64, ObservationQueryResult)) -> Self {
        let value_type = v.recipe.value_type.clone();
        Self {
            subject_id,
            points: v
                .points
                .into_iter()
                .map(|p| ObservationPointDto {
                    value: p.value.as_str().to_string(),
                    value_num: try_parse_numeric(&p.value, value_type.clone()),
                    observed_at: format_rfc3339_utc(p.observed_at),
                })
                .collect(),
            recipe: RecipeDto {
                id: v.recipe.id,
                kind: v.recipe.kind.to_string(),
                metric_code: v.recipe.metric_code.map(|code| code.0),
                metric_name: v.recipe.metric_name,
                unit: v.recipe.unit,
                value_type: v.recipe.value_type.map(|v| v.to_string()),
                visualization: v.recipe.visualization.map(|v| v.to_string()),
                status: v.recipe.status.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecipeDto {
    pub id: i64,
    pub kind: String,
    pub metric_code: Option<String>,
    pub metric_name: Option<String>,
    pub unit: Option<String>,
    pub value_type: Option<String>,
    pub visualization: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ObservationPointDto {
    pub value: String,
    pub value_num: Option<f64>,
    /// RFC3339 (UTC), 例如：2025-12-30T10:02:43.893518Z
    pub observed_at: String,
}

// =========================
// Record Observation
// =========================

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordObservationRequest {
    /// subject 全局 ID
    pub subject_id: i64,

    /// recipe 全局 ID
    pub recipe_id: i64,

    /// 观测值
    pub value: String,

    /// 观测发生的时间（RFC3339, 例如：2025-12-30T10:02:43.893518Z）
    pub observed_at: String,

    /// 数据来源信息
    pub source: SourceInput,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SourceInput {
    /// 来源类型：device / manual / import / system
    pub kind: String,

    /// 来源名称
    pub name: String,

    /// 可选元数据
    pub metadata: Option<JsonValue>,
}

impl RecordObservationRequest {
    pub fn to_internal(
        self,
    ) -> Result<(
        SubjectId,
        RecipeId,
        ObservationValue,
        OffsetDateTime,
        CreateDataSource,
    )> {
        let observed_at = parse_rfc3339(&self.observed_at)?;

        let source = CreateDataSource {
            kind: DataSourceKind::from(self.source.kind.as_str()),
            name: self.source.name,
            metadata: self.source.metadata,
        };

        Ok((
            SubjectId(self.subject_id),
            RecipeId(self.recipe_id),
            ObservationValue(self.value),
            observed_at,
            source,
        ))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecordObservationResponse {
    pub observation_id: i64,
    pub source_id: i64,
}

// =========================
// Upload Markdown Data Source
// =========================

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadMarkdownRequest {
    /// 数据来源类型 (device / manual / import / system)
    pub source_type: String,

    /// 数据来源名称
    pub source_name: String,

    /// 文件内容（文本形式）
    pub file_content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadMarkdownResponse {
    pub source_id: i64,
    pub source_type: String,
    pub source_name: String,
    /// 解析后的 Markdown JSON 数据
    pub parsed_data: JsonValue,
    pub created_at: String,
}

// =========================
// Extract Health Metrics with LLM
// =========================

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExtractHealthMetricsRequest {
    /// 健康检查 Markdown 内容
    pub content: String,

    /// 患者ID
    pub subject_id: i64,

    /// 数据来源类型
    #[serde(default)]
    pub source_type: Option<String>,

    /// 数据来源名称
    #[serde(default)]
    pub source_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct HealthMetric {
    /// 指标代码 (如: blood_glucose, blood_pressure 等)
    pub metric_code: String,

    /// 指标值（字符串格式，包含单位）
    pub value: String,

    /// 参考范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_range: Option<String>,

    /// 是否异常
    #[serde(default)]
    pub is_abnormal: bool,

    /// 异常说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abnormality_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExtractedHealthData {
    /// 患者基本信息
    pub patient_info: JsonValue,

    /// 提取的医疗指标
    pub metrics: Vec<HealthMetric>,

    /// 主要诊断/结论
    pub diagnoses: Vec<String>,

    /// 健康建议
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExtractHealthMetricsResponse {
    /// 提取的健康数据
    pub data: ExtractedHealthData,

    /// 数据来源ID（如果已保存）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,

    /// 插入的观测记录数
    pub records_inserted: usize,
}

fn try_parse_numeric(value: &ObservationValue, value_type: Option<MetricValueType>) -> Option<f64> {
    match value_type {
        Some(MetricValueType::Integer)
        | Some(MetricValueType::Float)
        | Some(MetricValueType::Decimal) => value.try_parse_f64(),
        _ => None,
    }
}
