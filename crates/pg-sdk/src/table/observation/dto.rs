// pg-sdk/src/core/observation.rs

use time::PrimitiveDateTime;

use crate::table::{
    data_source::dto::DataSourceId, metric::dto::MetricId, subject::dto::SubjectId,
};

/// Observation 表示：
/// 一次已经发生的“观测事实”
///
/// 核心原则：
/// - Observation 只描述“发生了什么”
/// - 不负责解释、不负责聚合、不负责展示
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    /// 系统内稳定的观测事实标识
    pub id: ObservationId,

    /// 观测主体（关于谁）
    pub subject_id: SubjectId,

    /// 被观测的指标（观测的是什么）
    pub metric_id: MetricId,

    /// 观测值（统一用字符串承载）
    ///
    /// 语义解释必须结合 Metric.value_type
    pub value: ObservationValue,

    /// 事实发生时间（设备时间 / 业务时间）
    pub observed_at: PrimitiveDateTime,

    /// 系统记录时间（写入时间）
    pub recorded_at: PrimitiveDateTime,

    /// 数据来源（可选）
    pub source_id: Option<DataSourceId>,
}

/// 记录 Observation 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordObservation {
    pub subject_id: SubjectId,
    pub metric_id: MetricId,
    pub value: ObservationValue,
    pub observed_at: PrimitiveDateTime,
    pub source_id: Option<DataSourceId>,
}

/// 按 Subject 查询 Observation 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListObservationBySubject {
    pub subject_id: SubjectId,
}

/// 按 Metric 查询 Observation 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListObservationByMetric {
    pub metric_id: MetricId,
}

/// 按时间范围查询 Observation 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListObservationByTimeRange {
    pub start: PrimitiveDateTime,
    pub end: PrimitiveDateTime,
    pub subject_id: Option<SubjectId>,
    pub metric_id: Option<MetricId>,
}

/// Observation 的强类型 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObservationId(pub i64);

/// Observation 的值
///
/// 设计说明：
/// - core 层不做数值解析
/// - 统一用字符串承载
/// - 解析/校验放在 service / business 层
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationValue(pub String);
