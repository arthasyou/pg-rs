// pg-sdk/src/core/observation.rs

use time::OffsetDateTime;

use crate::table::{metric::dto::MetricId, subject::dto::SubjectId};

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
    pub observed_at: OffsetDateTime,

    /// 系统记录时间（写入时间）
    pub recorded_at: OffsetDateTime,
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

impl Observation {
    /// 创建一个新的观测事实（尚未持久化）
    ///
    /// 注意：
    /// - 不在这里生成 id
    /// - 不在这里做业务校验
    pub fn new(
        subject_id: SubjectId,
        metric_id: MetricId,
        value: impl Into<String>,
        observed_at: OffsetDateTime,
        recorded_at: OffsetDateTime,
    ) -> Self {
        Self {
            id: ObservationId(0), // 占位，实际由存储层生成
            subject_id,
            metric_id,
            value: ObservationValue(value.into()),
            observed_at,
            recorded_at,
        }
    }
}
