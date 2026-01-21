use core::fmt;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::table::observation::dto::ObservationValue;

/// Metric 类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricKind {
    Primitive,
    Derived,
}

impl MetricKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetricKind::Primitive => "primitive",
            MetricKind::Derived => "derived",
        }
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MetricKind {
    fn from(value: &str) -> Self {
        match value {
            "primitive" => MetricKind::Primitive,
            "derived" => MetricKind::Derived,
            other => {
                let normalized = other.trim().to_ascii_lowercase();
                match normalized.as_str() {
                    "primitive" => MetricKind::Primitive,
                    "derived" => MetricKind::Derived,
                    _ => MetricKind::Primitive,
                }
            }
        }
    }
}

impl From<String> for MetricKind {
    fn from(value: String) -> Self {
        MetricKind::from(value.as_str())
    }
}

/// Metric 表示：
/// 一个“可被观测的指标定义”
///
/// 它描述的是：
/// - 观测的“是什么”
/// - 而不是“观测到的值”
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metric {
    /// 系统内稳定的指标标识
    pub id: MetricId,

    /// 指标类型
    pub kind: MetricKind,

    /// 稳定的语义代码（机器友好）
    /// 例如：blood_pressure_systolic
    pub code: MetricCode,

    /// 人类可读名称
    /// 例如：收缩压
    pub name: String,

    /// 单位（可选）
    /// 例如：mmHg、mg/dL
    pub unit: Option<String>,

    /// 指标值的类型定义
    pub value_type: MetricValueType,

    pub visualization: MetricVisualization,

    /// 创建时间（审计用途）
    pub created_at: OffsetDateTime,
}

impl Metric {
    /// 尝试将 observation 的值投影为“可比较的数轴值”
    ///
    /// 返回 Some(f64)：可用于排序 / 画图
    /// 返回 None：该指标不进入数值轴（如 Boolean / Text）
    pub fn try_parse_numeric(&self, value: &ObservationValue) -> Option<f64> {
        match self.value_type {
            MetricValueType::Integer => value.try_parse_f64(),
            MetricValueType::Float => value.try_parse_f64(),
            MetricValueType::Decimal => value.try_parse_f64(),
            MetricValueType::Boolean => None,
            MetricValueType::Text => None,
        }
    }
}

/// Metric summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummary {
    pub id: i64,
    pub metric_code: MetricCode,
    pub metric_name: String,
    pub unit: Option<String>,
    pub value_type: MetricValueType,
    pub visualization: MetricVisualization,
}

impl From<Metric> for MetricSummary {
    fn from(metric: Metric) -> Self {
        Self {
            id: metric.id.0,
            metric_code: metric.code,
            metric_name: metric.name,
            unit: metric.unit,
            value_type: metric.value_type,
            visualization: metric.visualization,
        }
    }
}

/// 创建 Metric 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateMetric {
    pub kind: MetricKind,
    pub code: MetricCode,
    pub name: String,
    pub unit: Option<String>,
    pub value_type: MetricValueType,
}

/// 查询 Metric 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListMetric {
    pub value_type: Option<MetricValueType>,
}

/// Metric 的强类型 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetricId(pub i64);

impl From<i64> for MetricId {
    fn from(value: i64) -> Self {
        MetricId(value)
    }
}

impl From<MetricId> for i64 {
    fn from(id: MetricId) -> Self {
        id.0
    }
}

/// 指标的稳定代码
///
/// 使用新类型而不是 String，
/// 是为了在 domain 中避免“裸字符串”横飞
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MetricCode(pub String);
impl AsRef<str> for MetricCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// 指标值的类型定义
///
/// 注意：
/// 这是“语义类型”，
/// 不是数据库类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricValueType {
    Integer,
    Float,
    Decimal,
    Boolean,
    Text,
}

/// 指标的可视化类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricVisualization {
    LineChart,
    BarChart,
    ValueList,
    SingleValue,
}

/// 指标的生命周期状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricStatus {
    /// 正常使用中
    Active,

    /// 已废弃（历史数据仍然有效）
    Deprecated,
}

impl fmt::Display for MetricValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricValueType::Integer => write!(f, "int"),
            MetricValueType::Float => write!(f, "float"),
            MetricValueType::Decimal => write!(f, "decimal"),
            MetricValueType::Boolean => write!(f, "bool"),
            MetricValueType::Text => write!(f, "text"),
        }
    }
}

impl Serialize for MetricValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MetricValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(MetricValueType::from(value))
    }
}

impl From<&str> for MetricValueType {
    fn from(value: &str) -> Self {
        match value {
            "int" => MetricValueType::Integer,
            "float" => MetricValueType::Float,
            "decimal" => MetricValueType::Decimal,
            "bool" => MetricValueType::Boolean,
            "text" => MetricValueType::Text,
            "string" => MetricValueType::Text,
            other => {
                let normalized = other.trim().to_ascii_lowercase();
                match normalized.as_str() {
                    "integer" => MetricValueType::Integer,
                    "boolean" => MetricValueType::Boolean,
                    _ => MetricValueType::Text,
                }
            }
        }
    }
}

impl From<String> for MetricValueType {
    fn from(value: String) -> Self {
        MetricValueType::from(value.as_str())
    }
}

impl MetricVisualization {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetricVisualization::LineChart => "line_chart",
            MetricVisualization::BarChart => "bar_chart",
            MetricVisualization::ValueList => "value_list",
            MetricVisualization::SingleValue => "single_value",
        }
    }
}

impl fmt::Display for MetricVisualization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for MetricVisualization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MetricVisualization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(MetricVisualization::from(value))
    }
}

impl From<&str> for MetricVisualization {
    fn from(value: &str) -> Self {
        match value {
            "line_chart" => MetricVisualization::LineChart,
            "bar_chart" => MetricVisualization::BarChart,
            "value_list" => MetricVisualization::ValueList,
            "single_value" => MetricVisualization::SingleValue,
            _ => {
                // 这里不 panic，是非常重要的设计选择
                // 防止脏数据把系统炸掉
                MetricVisualization::SingleValue
            }
        }
    }
}

impl From<String> for MetricVisualization {
    fn from(value: String) -> Self {
        MetricVisualization::from(value.as_str())
    }
}

impl fmt::Display for MetricStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricStatus::Active => write!(f, "active"),
            MetricStatus::Deprecated => write!(f, "deprecated"),
        }
    }
}

impl Serialize for MetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(MetricStatus::from(value))
    }
}

impl From<&str> for MetricStatus {
    fn from(value: &str) -> Self {
        match value {
            "active" => MetricStatus::Active,
            "deprecated" => MetricStatus::Deprecated,
            other => {
                let normalized = other.trim().to_ascii_lowercase();
                match normalized.as_str() {
                    "deprecated" => MetricStatus::Deprecated,
                    _ => MetricStatus::Active,
                }
            }
        }
    }
}

impl From<String> for MetricStatus {
    fn from(value: String) -> Self {
        MetricStatus::from(value.as_str())
    }
}
