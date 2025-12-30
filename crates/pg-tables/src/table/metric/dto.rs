use core::fmt;

use time::PrimitiveDateTime;

use crate::table::observation::dto::ObservationValue;

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

    /// 指标当前状态
    pub status: MetricStatus,

    /// 创建时间（审计用途）
    pub created_at: PrimitiveDateTime,
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

/// 创建 Metric 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateMetric {
    pub code: MetricCode,
    pub name: String,
    pub unit: Option<String>,
    pub value_type: MetricValueType,
}

/// 查询 Metric 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListMetric {
    pub status: Option<MetricStatus>,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl fmt::Display for MetricStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricStatus::Active => write!(f, "active"),
            MetricStatus::Deprecated => write!(f, "deprecated"),
        }
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

impl Metric {
    /// 判断该指标是否仍然可用于新观测
    pub fn is_active(&self) -> bool {
        matches!(self.status, MetricStatus::Active)
    }
}
