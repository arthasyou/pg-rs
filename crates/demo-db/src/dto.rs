//! Business-level DTOs for demo-db (health example)
//!
//! 规则：
//! - ID 类型必须与数据库保持 1:1
//! - 数据库是 i64，这里就用 i64
//! - 不引入额外抽象

use pg_tables::table::{
    metric::dto::{MetricCode, MetricId, MetricValueType},
    observation::dto::ObservationValue,
};
use time::PrimitiveDateTime;

/// =========================
/// 业务输入 DTO
/// =========================

/// 记录一次健康观测（跨表业务行为）
pub struct RecordObservationRequest {
    pub subject_id: i64,
    pub metric_id: i64,

    /// 业务层只关心“数值是什么”
    pub value: f64,

    /// 观测发生的时间
    pub observed_at: PrimitiveDateTime,

    /// 数据来源（设备 / 手工 / 第三方）
    pub source: Option<String>,
}

/// 查询观测数据
pub struct QueryObservationsRequest {
    pub subject_id: i64,
    pub metric_id: Option<i64>,

    pub from: Option<PrimitiveDateTime>,
    pub to: Option<PrimitiveDateTime>,
}

/// =========================
/// 业务输出 DTO
/// =========================

/// 业务视角的单条观测结果
pub struct ObservationView {
    pub metric: MetricView,
    pub value: ObservationValue,
    pub observed_at: PrimitiveDateTime,
    pub source: Option<String>,
}

pub struct MetricView {
    pub id: MetricId,
    pub code: MetricCode,
    pub name: String,
    pub unit: Option<String>,
    pub value_type: MetricValueType,
}

/// 观测结果列表
pub struct ObservationList {
    pub subject_id: i64,
    pub items: Vec<ObservationView>,
}
