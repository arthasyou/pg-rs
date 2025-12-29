//! Business-level DTOs for demo-db (health example)
//!
//! 规则：
//! - ID 类型必须与数据库保持 1:1
//! - 数据库是 i64，这里就用 i64
//! - 不引入额外抽象

use pg_tables::table::{
    metric::dto::{MetricId, MetricView},
    observation::dto::{ObservationPoint, ObservationValue},
    subject::dto::SubjectId,
};
use time::PrimitiveDateTime;

/// =========================
/// 业务输入 DTO
/// =========================

/// 记录一次健康观测（跨表业务行为）
pub struct RecordObservationRequest {
    pub subject_id: SubjectId,
    pub metric_id: MetricId,

    /// 业务层只关心“数值是什么”
    pub value: ObservationValue,

    /// 观测发生的时间
    pub observed_at: PrimitiveDateTime,

    /// 数据来源（设备 / 手工 / 第三方）
    pub source: Option<String>,
}

/// 查询观测数据
pub struct QueryObservationSeries {
    pub subject_id: SubjectId,
    pub metric_id: MetricId,
}

/// =========================
/// 业务输出 DTO
/// =========================

/// 业务视角的单条观测结果
#[derive(Debug, Clone)]
pub struct QueryObservationResponse {
    pub metric: MetricView,
    pub points: Vec<ObservationPoint>,
}
